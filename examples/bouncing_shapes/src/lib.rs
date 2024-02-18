extern crate quipi_2d as quipi;
extern crate nalgebra_glm as glm;

use quipi::components::CQuad;
pub use quipi::{
    components::{
        CScene,
        CRGBA,
        CTransform2D
    },
    core::canvas,
    schemas::{
        ISchema,
        SchemaScene2D,
        SchemaCamera2D,
        camera2d::DEFAULT_CAMERA
    },
    systems::scene::load_scene_2d,
    FrameResponse,
    FrameState,
    QuiPiApp,
    QuiPiWindow,
    Registry,
    VersionedIndex
};
pub use quipi_core::opengl::textures::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod systems;

use quipi_core::{
    components::CDrawable, core::canvas::get_dimensions, ecs::EMQuery, opengl::capabilities::{
        gl_blending_func,
        gl_enable,
        GLBlendingFactor,
        GLCapability
    },
    rendering::batch::BatchRenderer,
    resources::{
        shader::UniformVariable,
        RShader,
    }, schemas::SchemaShader
};
use systems::{
    *,
    spawner::RectSpawner
};

pub struct BouncingShapes {
    spawner: Option<RectSpawner>,
    scene: Option<VersionedIndex>,

    batch_renderer: Option<BatchRenderer<500, CQuad>>
}

impl BouncingShapes {
    pub fn new() -> Self {
        BouncingShapes {
            spawner: None,
            scene: None,

            batch_renderer: None
        }
    }
}

impl quipi::QuiPiApp for BouncingShapes {
    fn init(
        &mut self,
        registry: &mut Registry,
        _winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scene = load_scene_2d(
            "bouncing_shapes",
            scene_schema()
        )?;

        self.scene = Some(scene.build_entity(registry)?);

        let mut spawner = RectSpawner::new()?;
        for _ in 0..1000 {
            spawner.spawn(registry)?;
        }

        self.spawner = Some(spawner);
        self.batch_renderer = Some(BatchRenderer::new());

        Ok(())
    }

    fn handle_frame(
        &mut self,
        registry: &mut Registry,
        frame_state: &mut FrameState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        if self.scene.is_none() {
            return Err("There is no scene defined".into());
        };
        
        let Some(scene) = registry.entities.get::<CScene>(&self.scene.unwrap()) else {
            return Err("Invalid scene".into());
        };

        frame_state.clear_color = scene.color;

        // handle input
        let frame_response = handle_input(
            frame_state,
            registry,
            self.spawner.as_mut().unwrap()
        )?;

        // update
        update(registry, frame_state)?;
        
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);
        if let Some(batch_renderer) = &mut self.batch_renderer {
            let entities = EMQuery::<CQuad, CTransform2D, CDrawable>::query_all(registry);
            if entities.len() > 0 {
                let drawable = registry.entities.get::<CDrawable>(&entities[0]).unwrap();
                let shader_id = drawable.shader;
                
                if registry.get_resource::<RShader>(drawable.shader).is_none() {
                    return Err("tried to use a shader that is not loaded".into())
                };

                let view = glm::look_at(
                    &glm::vec3(0.0, 0.0, 0.0), 
                    &(glm::vec3(0.0, 0.0, 0.0) + glm::vec3(0.0, 0.0, -1.0)),
                    &glm::vec3(0.0, 1.0, 0.0)
                );

                let (_x, _y, width, height) = get_dimensions();
                let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, 0.0, 0.2);

                batch_renderer.reset_info();
                batch_renderer.begin_batch();
                for entity in entities {
                    let transform = registry.entities.get::<CTransform2D>(&entity).unwrap();
                    let model = transform.to_matrix();
                    let mvp = projection * view * model;

                    let Some(quad) = registry.entities.get_mut::<CQuad>(&entity) else { continue; };
                    quad.mvp = mvp;

                    let quad = registry.entities.get::<CQuad>(&entity).unwrap();
                    let drawable = registry.entities.get::<CDrawable>(&entity).unwrap();
                    let texture = match drawable.texture {
                        Some(id) => registry.get_resource(id),
                        _ => None
                    };

                    batch_renderer.draw_mesh(quad, registry.get_resource(shader_id).unwrap(), texture);
                }
                batch_renderer.end_batch();
                batch_renderer.flush_batch(registry.get_resource(shader_id).unwrap());

                frame_state.render_info = batch_renderer.render_info.clone();
            }
        }

        // draw the entity count
        let entity_count = registry.entities.count();
        frame_state.text_render.color = glm::vec4(0.0, 0.0, 0.0, 1.0);
        frame_state.text_render.scale = 0.6;
        frame_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(20.0, 20.0)
        );
        frame_state.text_render.draw(
            format!("draw calls: {}", frame_state.render_info.num_draw_calls),
            glm::vec2(20.0, 40.0)
        );
        frame_state.text_render.draw(
            format!("fps: {}", frame_state.debug_info.fps as u32),
            glm::vec2(20.0, 60.0)
        );
        frame_state.text_render.draw(
            format!("ms: {}", frame_state.debug_info.ms as u32),
            glm::vec2(20.0, 80.0)
        );

        Ok(frame_response)
    }
}

fn scene_schema() -> SchemaScene2D {
    SchemaScene2D {
        name: "bouncing_shapes".to_string(),
        clr_color: glm::vec4(1.0, 1.0, 0.8, 1.0),
        cameras: vec![camera_schema()],
        entities: vec![],
        shaders: vec![SchemaShader {
            name: "sprite".to_string(),
            uniforms: vec![
                // UniformVariable::ModelMatrix("model".into()),
                UniformVariable::ViewMatrix("view".into()),
                UniformVariable::ProjectionMatrix("projection".into())
            ]
        }],
        textures: vec!["Sprite-0001.png".into()]
    }
}

fn camera_schema() -> SchemaCamera2D {
    SchemaCamera2D {
        right: WIDTH as f32,
        top: HEIGHT as f32,
        ..SchemaCamera2D::default()
    }
}

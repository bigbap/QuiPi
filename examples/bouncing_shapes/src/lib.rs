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

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod systems;

use quipi_core::{
    components::{
        CName,
        CTexture
    },
    core::canvas::get_dimensions,
    ec_store::EMQuery,
    opengl::capabilities::{
        gl_blending_func,
        gl_enable,
        GLBlendingFactor,
        GLCapability
    },
    rendering::batch::BatchRenderer,
    resources::{
        shader::UniformVariable,
        RShader, RTexture
    },
    schemas::SchemaShader
};
use systems::{
    *,
    spawner::RectSpawner
};

pub struct BouncingShapes {
    spawner: Option<RectSpawner>,
    scene: Option<VersionedIndex>,
    camera: Option<VersionedIndex>,

    batch_renderer: Option<BatchRenderer<500, CQuad>>
}

impl BouncingShapes {
    pub fn new() -> Self {
        BouncingShapes {
            spawner: None,
            scene: None,
            camera: None,

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

        let camera_name = scene.cameras.first().unwrap().name.clone();

        self.scene = Some(scene.build(registry)?);
        self.camera = Some(registry.entities.query::<CName>(camera_name)
            .first()
            .unwrap()
            .to_owned());

        let mut spawner = RectSpawner::new(self.camera.unwrap())?;
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
        
        let scene = self.scene.unwrap();

        if let Some(color) = registry.entities.get::<CRGBA>(&scene) {
            frame_state.clear_color = *color;
        }

        // handle input
        let frame_response = handle_input(
            frame_state,
            registry,
            self.spawner.as_mut().unwrap()
        )?;

        // update
        update(registry, frame_state)?;

        // draw batch
        let shader_id = registry.resources.query(CName { name: "sprite".into() });
        let shader_id = shader_id.first().unwrap();
        let shader = registry.resources.get::<RShader>(shader_id).unwrap();

        let texture_id = registry.resources.query::<CName>(CName { name: "Sprite-0001.png".into() });
        let texture = registry.resources.get::<RTexture>(&texture_id.first().unwrap());
        
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);
        if let Some(batch_renderer) = &mut self.batch_renderer {
            let entities = EMQuery::<CQuad, CTransform2D, CTexture>::query_all(registry);

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
                let Some(transform) = registry.entities.get::<CTransform2D>(&entity) else { continue; };
                let model = transform.to_matrix();
                let mvp = projection * view * model;

                let Some(quad) = registry.entities.get_mut::<CQuad>(&entity) else { continue; };

                quad.mvp = mvp;

                batch_renderer.draw_mesh(quad, shader, texture);
            }
            batch_renderer.end_batch();
            batch_renderer.flush_batch(shader);

            frame_state.render_info = batch_renderer.render_info.clone();
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
        name: CScene { name: "bouncing_shapes".to_string() },
        clr_color: CRGBA { value: [1.0, 1.0, 0.8, 1.0] },
        cameras: vec![camera_schema()],
        entities: vec![],
        shaders: vec![SchemaShader {
            name: CName { name: "sprite".into() },
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
        name: CName { name: DEFAULT_CAMERA.to_string() },
        left: 0.0,
        right: WIDTH as f32,
        bottom: 0.0,
        top: HEIGHT as f32,
        near: 0.0,
        far: 0.2,
        transform: CTransform2D::default(),
    }
}

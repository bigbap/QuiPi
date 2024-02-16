extern crate quipi_2d as quipi;
extern crate nalgebra_glm as glm;

use quipi::{components::CRect, DrawMode};
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

use quipi_core::{components::{CName, CTexture}, ec_store::EMQuery, opengl::{self, capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability}, draw::DrawBuffer}, rendering::{batch::{BatchDynamic, BatchStatic, IMesh}, vertex::Vertex}, resources::{shader::UniformVariable, RShader, RTexture}, schemas::SchemaShader};
use systems::{
    *,
    spawner::RectSpawner
};

pub struct BouncingShapes {
    spawner: Option<RectSpawner>,
    scene: Option<VersionedIndex>,
    camera: Option<VersionedIndex>,

    batch_dynamic: Option<BatchDynamic<CRect>>,
    batch_static: Option<BatchStatic<CRect>>
}

impl BouncingShapes {
    pub fn new() -> Self {
        BouncingShapes {
            spawner: None,
            scene: None,
            camera: None,
            batch_dynamic: None,
            batch_static: None
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

        // self.spawner = Some(RectSpawner::new(self.camera.unwrap())?);

        let mut spawner = RectSpawner::new(self.camera.unwrap())?;
        for _ in 0..1000 {
            spawner.spawn(registry)?;
        }

        ////////////////////////////////////////////////////////
        
        let entities = EMQuery::<CRect, CTransform2D>::query_all(registry);

        let view = glm::look_at(
            &glm::vec3(0.0, 0.0, 0.0), 
            &(glm::vec3(0.0, 0.0, 0.0) + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0)
        );
        let projection = glm::ortho(0.0, WIDTH as f32, 0.0, HEIGHT as f32, 0.0, 0.2);

        let mut vertices = Vec::<Vertex>::new();
        for entity in entities {
            let transform = registry.entities.get::<CTransform2D>(&entity).unwrap();
            let model = transform.to_matrix().0;
            let mvp = projection * view * model;

            let rect = registry.entities.get_mut::<CRect>(&entity).unwrap();
            for mut vertex in rect.vertices() {
                let tmp = mvp * glm::vec4(
                    vertex.position.x,
                    vertex.position.y,
                    vertex.position.z,
                    1.0
                );

                vertex.position = glm::vec3(tmp.x, tmp.y, tmp.z);
                vertices.push(vertex);
            }
        }

        self.batch_static = Some(BatchStatic::<CRect>::new(1000, vertices));

        //////////////////////////////////////////////////////
        
        self.batch_dynamic = Some(BatchDynamic::<CRect>::new(200));

        self.spawner = Some(spawner);

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
        let shader_id = registry.resources.query(CName { name: "default".into() });
        let shader_id = shader_id.first().unwrap();
        let shader = registry.resources.get::<RShader>(shader_id).unwrap();

        let texture_id = registry.resources.query::<CName>(CName { name: "Sprite-0001.png".into() });
        let texture = registry.resources.get::<RTexture>(&texture_id.first().unwrap());
        
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);
        if let Some(batch) = self.batch_dynamic.as_mut() {
            let entities = EMQuery::<CRect, CTransform2D, CTexture>::query_all(registry);

            let view = glm::look_at(
                &glm::vec3(0.0, 0.0, 0.0), 
                &(glm::vec3(0.0, 0.0, 0.0) + glm::vec3(0.0, 0.0, -1.0)),
                &glm::vec3(0.0, 1.0, 0.0)
            );
            let projection = glm::ortho(0.0, WIDTH as f32, 0.0, HEIGHT as f32, 0.0, 0.2);

            let mut vertices = Vec::<Vertex>::new();
            for entity in entities {
                let transform = registry.entities.get::<CTransform2D>(&entity).unwrap();
                let model = transform.to_matrix().0;
                let mvp = projection * view * model;

                let rect = registry.entities.get_mut::<CRect>(&entity).unwrap();
                for mut vertex in rect.vertices() {
                    let tmp = mvp * glm::vec4(
                        vertex.position.x,
                        vertex.position.y,
                        vertex.position.z,
                        1.0
                    );

                    vertex.position = glm::vec3(tmp.x, tmp.y, tmp.z);
                    vertices.push(vertex);
                }
            }

            batch.update(vertices);

            shader.program.use_program();
            // shader.program.set_mat4("view", &view);
            // shader.program.set_mat4("projection", &projection);

            texture.unwrap().0.use_texture(0);
            shader.program.set_int("u_texture", 0);

            batch.vao.bind();
            opengl::draw::gl_draw(
                DrawBuffer::Elements,
                DrawMode::Triangles, // TODO: this is hardcoded
                batch.vertex_capacity as i32
            );
            batch.vao.unbind();
        }

        if let Some(batch) = &self.batch_static {
            shader.program.use_program();

            texture.unwrap().0.use_texture(0);
            shader.program.set_int("u_texture", 0);

            batch.vao.bind();
            opengl::draw::gl_draw(
                DrawBuffer::Elements,
                DrawMode::Triangles, // TODO: this is hardcoded
                batch.vertex_capacity as i32
            );
            batch.vao.unbind();
        }

        // draw the entity count
        let entity_count = registry.entities.count();
        frame_state.text_render.color = glm::vec4(0.9, 0.0, 0.3, 0.8);
        frame_state.text_render.scale = 0.7;
        frame_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(20.0, 30.0)
        );
        frame_state.text_render.draw(
            format!("fps: {}", frame_state.debug_info.fps as u32),
            glm::vec2(20.0, 60.0)
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
            name: CName { name: "default".into() },
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

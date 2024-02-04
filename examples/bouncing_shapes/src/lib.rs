extern crate quipi;
extern crate nalgebra_glm as glm;

use quipi::{
    Registry,
    VersionedIndex,
    resources::{
        register_resources,
        Shader,
        shader::UniformVariable
    },
    systems::rendering::canvas,
    components::{
        register_components,
        CTransform,
    },
    wrappers::{
        opengl::shader::ShaderProgram,
        sdl2::window::QuiPiWindow,
    },
    AppState,
    FrameResponse,
    schema::{
        SchemaCamera,
        camera::{
            CameraParams,
            CameraKind, DEFAULT_CAMERA_TAG
        },
        scene::{
            DEFAULT_SHADER,
            DEFAULT_SHADER_TAG,
            DEFAULT_SHADER_UNIFORM
        },
        rect::DEFAULT_RECT_TAG
    }
};

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

mod systems;

use systems::{
    *,
    spawner::RectSpawner
};

pub struct MyGame {
    registry: Registry,

    camera: VersionedIndex,
    spawner: Option<RectSpawner>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;

        register_resources(&mut registry);
        register_components(&mut registry);

        let camera = camera_schema()
            .build_camera(&mut registry)?;

        Ok(MyGame {
            registry,
            camera,
            spawner: None
        })
    }
}

impl quipi::QuiPiApp for MyGame {
    fn init(
        &mut self,
        _winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new(DEFAULT_SHADER)?;
        let shader_id = self.registry.create_resource(DEFAULT_SHADER_TAG, Shader {
            program: shader,
            uniforms: vec![
                UniformVariable::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
            ]
        })?;

        let mut spawner = RectSpawner::new(&shader_id)?;

        create_shapes(&mut self.registry, &mut spawner)?;

        self.spawner = Some(spawner);
        
        Ok(())
    }

    fn handle_frame(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        // handle input
        let frame_response = s_handle_input(
            app_state,
            &mut self.registry,
            self.spawner.as_mut().unwrap()
        )?;

        s_update(
            app_state,
            &mut self.registry,
        )?;

        // render
        s_draw_frame(
            &mut self.registry,
            &self.camera
        )?;

        // draw the entity count
        let (_x, _y, width, height) = canvas::get_dimensions();
        let entity_count = self.registry.entity_count();
        app_state.text_render.color = glm::vec3(1.0, 1.0, 1.0);
        app_state.text_render.scale = 0.7;
        app_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
        );

        Ok(frame_response)
    }
}

fn create_shapes(
    registry: &mut Registry,
    spawner: &mut RectSpawner
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..10 {
        spawner.spawn(registry)?;
    }

    Ok(())
}

fn camera_schema() -> SchemaCamera {
    SchemaCamera {
        tag: DEFAULT_CAMERA_TAG.to_string(),
        params: CameraParams {
            kind: CameraKind::Cam2D,
            left: 0.0,
            right: WIDTH as f32,
            bottom: 0.0,
            top: HEIGHT as f32,
            near: 0.0,
            far: 0.2,
            ..CameraParams::default()
        },
        transform: CTransform::default(),
        entities: vec![DEFAULT_RECT_TAG.to_string()]
    }
}

extern crate quipi;
extern crate nalgebra_glm as glm;

use quipi::{
    components::{
        register_components, CTag, CTransform, CRGBA
    }, resources::register_resources, schemas::{
        camera::{
            CameraKind, CameraParams, DEFAULT_CAMERA_TAG
        }, rect::DEFAULT_RECT_TAG, ISchema, SchemaCamera, SchemaRect, SchemaScene, SchemaShader
    }, systems::{rendering::canvas, scene::load_scene}, wrappers::sdl2::window::QuiPiWindow, AppState, FrameResponse, Registry, VersionedIndex
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

    spawner: Option<RectSpawner>,
    scene: SchemaScene,
    camera: Option<VersionedIndex>
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;

        register_resources(&mut registry);
        register_components(&mut registry);
        
        let scene = load_scene(
            "bouncing_shapes",
            scene_schema()
        )?;

        Ok(MyGame {
            registry,
            spawner: None,
            scene,
            camera: None
        })
    }
}

impl quipi::QuiPiApp for MyGame {
    fn init(
        &mut self,
        _winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.scene.build(&mut self.registry)?;

        self.camera = Some(self.registry.entities.query::<CTag>(
            self.scene.cameras.first().unwrap().tag.clone()
        ).first().unwrap().to_owned());

        self.spawner = Some(RectSpawner::new(self.camera.unwrap())?);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        app_state.clear_color = self.scene.clr_color;

        // handle input
        let frame_response = s_handle_input(
            app_state,
            &mut self.registry,
            self.spawner.as_mut().unwrap(),
            &mut self.scene
        )?;

        s_update(
            app_state,
            &mut self.registry,
        )?;

        // render
        s_draw_frame(
            &mut self.registry,
            &self.camera.unwrap()
        )?;

        // draw the entity count
        let (_x, _y, width, height) = canvas::get_dimensions();
        let entity_count = self.registry.entities.count();
        app_state.text_render.color = glm::vec3(1.0, 1.0, 1.0);
        app_state.text_render.scale = 0.7;
        app_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
        );

        Ok(frame_response)
    }

    fn handle_editor(
        &mut self,
        app_state: &AppState,
        editor: &mut quipi::systems::editor::SceneEditor
    ) -> Result<(), Box<dyn std::error::Error>> {
        editor.update(&mut self.registry, app_state)
    }
}

fn scene_schema() -> SchemaScene {
    SchemaScene {
        tag: CTag { tag: "bouncing_shapes".to_string() },
        clr_color: CRGBA { r: 0.0, g: 0.3, b: 0.5, a: 1.0 },
        cameras: vec![camera_schema()],
        rects: vec![rect_schema()],
        shaders: vec![shader_schema()]
    }
}

fn camera_schema() -> SchemaCamera {
    SchemaCamera {
        tag: CTag { tag:DEFAULT_CAMERA_TAG.to_string() },
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
        entities: vec![CTag { tag: DEFAULT_RECT_TAG.to_string() }]
    }
}

fn shader_schema() -> SchemaShader {
    SchemaShader::default()
}

fn rect_schema() -> SchemaRect {
    SchemaRect {
        instances: vec![],
        ..SchemaRect::default()
    }
}

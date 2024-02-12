extern crate quipi_2d as quipi;
extern crate nalgebra_glm as glm;

pub use quipi::{
    components::{
        CScene,
        CRGBA,
        CTransform2D
    },
    rendering::canvas,
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

use quipi_core::{components::CName, resources::shader::UniformVariable, schemas::SchemaShader};
use systems::{
    *,
    spawner::RectSpawner
};

pub struct BouncingShapes {
    spawner: Option<RectSpawner>,
    scene: Option<VersionedIndex>,
    camera: Option<VersionedIndex>
}

impl BouncingShapes {
    pub fn new() -> Self {
        BouncingShapes {
            spawner: None,
            scene: None,
            camera: None
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

        self.spawner = Some(RectSpawner::new(self.camera.unwrap())?);

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

        // draw the entity count
        let (_x, _y, width, height) = canvas::get_dimensions();
        let entity_count = registry.entities.count();
        frame_state.text_render.color = glm::vec3(0.0, 0.0, 0.0);
        frame_state.text_render.scale = 0.7;
        frame_state.text_render.draw(
            format!("entities: {}", entity_count),
            glm::vec2(width as f32 - 120.0, height as f32 - 30.0)
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
                UniformVariable::MVPMatrix("mvpMatrix".into())
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

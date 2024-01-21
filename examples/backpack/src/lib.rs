extern crate nalgebra_glm as glm;

use engine::components::{
    register_components,
    CModelNode,
    CTransform,
    CModelMatrix,
    CMaterial
};
use engine::resources::{
    register_resources,
    Shader
};
use engine::systems::{
    load_gltf::s_create_model_from_gltf,
    mvp_matrices::s_set_model_matrix
};
use engine::{
    Game,
    VersionedIndex,
    Registry,
    gfx::object_loader::{
        ObjectConfig,
        load_obj_file
    },
    gfx::ElementArrayMesh
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

mod config;

pub static WIDTH: u32 = 800;
pub static HEIGHT: u32 = 600;

pub struct MyGame {
    registry: Registry,
    shader: Option<VersionedIndex>,
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;
        
        register_components(&mut registry);
        register_resources(&mut registry);

        Ok(Self {
            registry,
            shader: None
        })
    }
}

impl Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let asset_path = config::asset_path()?.into_os_string().into_string().unwrap();
        self.shader = Some(self.registry.create_resource(Shader::new(
            &format!("{}/shaders/backpack", asset_path),
            vec![]
        )?)?);

        load_backpack(&mut self.registry)?;

        Ok(())
    }

    fn handle_frame(
        &mut self,
        event_pump: &mut sdl2::EventPump
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return Ok(None),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => engine::gfx::view::adjust_viewport_dims(w, h),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
                _ => ()
            }
        }

        Ok(Some(()))
    }
}

fn load_backpack(
    registry: &mut Registry
) -> Result<(), Box<dyn std::error::Error>> {
    let asset_path = config::asset_path()?.into_os_string().into_string().unwrap();
    s_create_model_from_gltf(&format!("{}/objects/scene.gltf", asset_path))?;

    // let (models_obj, _materials_obj) = load_obj_file(format!("{}/objects/backpack.obj", asset_path))?;
    // let model_configs = ObjectConfig::from_obj(models_obj)?;
    //
    // for config in model_configs.iter() {
    //     let mesh = ElementArrayMesh::new(&config.indices)?;
    //     mesh
    //         .create_vbo_at(&config.points, 0, 3)?
    //         .create_vbo_at(&config.texture_coords, 2, 2)?;
    //
    //     let entity = registry.create_entity("backpack")?
    //         .with(CMesh { mesh })?
    //         .with(CTransform {
    //             translate: Some(glm::vec3(0.0, 0.0, 0.0)),
    //             ..CTransform::default()
    //         })?
    //         .with(CModelMatrix::default())?
    //         .with(CMaterial::default())?
    //         .done()?;
    //
    //     s_set_model_matrix(&entity, registry);
    // }

    Ok(())
}

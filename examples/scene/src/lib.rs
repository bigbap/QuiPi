extern crate engine;
extern crate gl;
extern crate nalgebra_glm as glm;

use engine::{
    resources::{
        texture::TextureType,
        Shader,
        Camera3D
    },
    VersionedIndex,
    gfx::{
        Material,
        material::MaterialPart,
        object_loader::{
            load_obj_file,
            ObjectConfig
        }
    }
};
use sdl2::{
    EventPump,
    keyboard::Keycode,
    event::{
        Event,
        WindowEvent
    }
};

mod components;
mod systems;
mod resources;
mod config;
mod scene;

pub use config::CONFIG;

use scene::*;

type Crate = VersionedIndex;
type Light = VersionedIndex;

pub struct MyGame {
    registry: engine::Registry,
    timer: std::time::Instant,
   
    shader: Option<VersionedIndex>,
    crates: Vec<Crate>,
    camera: VersionedIndex,
    direction_light: Option<Light>,
    point_light: Option<Light>,
    spot_light: Option<Light>,

    direction_light_on: bool,
    point_light_on: bool,
    spot_light_on: bool,

    last_frame: f32,
    _has_control: bool
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let timer = std::time::Instant::now();
        let camera = create_camera(&mut registry)?;

        Ok(Self {
            registry,
            timer,

            shader: None,
            crates: vec![],
            camera,
            direction_light: None,
            point_light: None,
            spot_light: None,

            direction_light_on: true,
            point_light_on: true,
            spot_light_on: true,
            
            last_frame: timer.elapsed().as_millis() as f32 / 1000.0,
            _has_control: true
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let shader = self.registry.create_resource(
            Shader::new(&format!("{}shaders/lighting", CONFIG.asset_path))?
        )?;
        let light_shader = self.registry.create_resource(
            Shader::new(&format!("{}shaders/light_source", CONFIG.asset_path))?
        )?;
        
        let diffuse = create_texture(
            &mut self.registry,
            &format!("{}objects/textures/container.png", CONFIG.asset_path),
            TextureType::Diffuse,
            0
        )?;
        let specular = create_texture(
            &mut self.registry,
            &format!("{}objects/textures/container_specular.png", CONFIG.asset_path),
            TextureType::Specular,
            1
        )?;

        self.crates = create_crates(
            &mut self.registry,
            shader,
            self.camera,
            Material {
                diffuse: MaterialPart::Texture(diffuse),
                specular: MaterialPart::Texture(specular),
                shininess: 0.6 * 128.0,
                ..Material::default()
            }
        )?;

        let (models_obj, _materials_obj) = load_obj_file(format!("{}objects/sphere.obj", CONFIG.asset_path))?;
        let model_configs = ObjectConfig::from_obj(models_obj)?;
        self.direction_light = Some(directional_light(
            &mut self.registry,
            light_shader,
            shader,
            self.camera,
            model_configs.get(0).unwrap()
        )?);
        self.point_light = Some(point_light(
            &mut self.registry,
            light_shader,
            shader,
            self.camera,
            model_configs.get(0).unwrap()
        )?);
        self.spot_light = Some(spot_light(
            &mut self.registry,
            light_shader,
            shader,
            self.camera,
            model_configs.get(0).unwrap()
        )?);

        self.shader = Some(shader);

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let ticks = self.ticks();
        let delta = ticks - self.last_frame;
        
        self.last_frame = ticks;
        
        let camera = self.registry.get_resource_mut::<Camera3D>(&self.camera).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return Ok(None),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => engine::gfx::view::adjust_viewport_dims(w, h),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
                Event::KeyDown { keycode: Some(Keycode::Num1), repeat: false, .. } => self.direction_light_on = !self.direction_light_on,
                Event::KeyDown { keycode: Some(Keycode::Num2), repeat: false, .. } => self.point_light_on = !self.point_light_on,
                Event::KeyDown { keycode: Some(Keycode::Num3), repeat: false, .. } => self.spot_light_on = !self.spot_light_on,

                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => camera.move_forward = true,
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => camera.move_backward = true,
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => camera.move_left = true,
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => camera.move_right = true,
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => camera.move_forward = false,
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => camera.move_backward = false,
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => camera.move_left = false,
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => camera.move_right = false,
                Event::MouseMotion { xrel, yrel, .. } => {
                    let sensitivity = 0.1;
                    camera.rotate(
                        xrel as f32 * sensitivity,
                        yrel as f32 * sensitivity
                    );
                },
                _event => ()
            };
        }

        camera.apply_move(5.0 * delta);
        let camera_pos = camera.position_tup();
        let camera_dir = camera.front_tup();

        engine::gfx::buffer::clear_buffer(Some((0.02, 0.02, 0.02, 1.0)));
        
        if self.direction_light_on {
            systems::draw(&self.direction_light.unwrap(), &self.registry)?;
        }
        if self.point_light_on {
            systems::draw(&self.point_light.unwrap(), &self.registry)?;
        }

        let shader = self.registry.get_resource::<Shader>(&self.shader.unwrap()).unwrap().program();
        shader.set_int("dirLightOn", self.direction_light_on as i32);
        shader.set_int("pointLightOn", self.point_light_on as i32);
        shader.set_int("spotLightOn", self.spot_light_on as i32);
        shader.set_float_3("spotLight.position", camera_pos);
        shader.set_float_3("spotLight.direction", camera_dir);
        
        for entity in self.crates.iter() {
            systems::update_entity(entity, &self.registry);
            systems::draw(entity, &self.registry)?;
        }

        Ok(Some(()))
    }
}

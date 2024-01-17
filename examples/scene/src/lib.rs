extern crate engine;
extern crate nalgebra_glm as glm;

use engine::{
    resources::Shader,
    VersionedIndex,
    gfx::object_loader::{
        load_obj_file,
        ObjectConfig
    },
    components::{
        CMaterial,
        material::MaterialPart,
        CPosition,
        CGizmo3D,
        CVelocity
    },
    systems::{
        movement::s_apply_velocity,
        rotation::{
            s_update_angles,
            s_rotate
        }
    },
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

use scene::*;

type Crate = VersionedIndex;
type Light = VersionedIndex;

const CAMERA_SPEED: f32 = 5.0;

pub struct MyGame {
    registry: engine::Registry,
    timer: std::time::Instant,
   
    shader: Option<VersionedIndex>,
    light_shader: Option<VersionedIndex>,
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
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let timer = std::time::Instant::now();
        let camera = create_camera(
            &mut registry,
            width as f32,
            height as f32
        )?;

        Ok(Self {
            registry,
            timer,

            shader: None,
            light_shader: None,
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
        let asset_path = config::asset_path()?.into_os_string().into_string().unwrap();
        let shader = self.registry.create_resource(
            Shader::new(&format!("{}/shaders/lighting", asset_path))?
        )?;
        let light_shader = self.registry.create_resource(
            Shader::new(&format!("{}/shaders/light_source", asset_path))?
        )?;
        
        let diffuse = create_texture(
            &mut self.registry,
            &format!("{}/objects/textures/container.png", asset_path),
        )?;
        let specular = create_texture(
            &mut self.registry,
            &format!("{}/objects/textures/container_specular.png", asset_path),
        )?;

        self.crates = create_crates(
            &mut self.registry,
            shader,
            self.camera,
            CMaterial {
                diffuse: MaterialPart::Texture(diffuse),
                specular: MaterialPart::Texture(specular),
                shininess: 0.6 * 128.0,
                ..CMaterial::default()
            }
        )?;

        let (models_obj, _materials_obj) = load_obj_file(format!("{}/objects/sphere.obj", asset_path))?;
        let model_configs = ObjectConfig::from_obj(models_obj)?;
        self.direction_light = Some(directional_light(
            &mut self.registry,
            shader,
            model_configs.get(0).unwrap()
        )?);
        self.point_light = Some(point_light(
            &mut self.registry,
            shader,
            model_configs.get(0).unwrap()
        )?);
        self.spot_light = Some(spot_light(
            &mut self.registry,
            shader,
            model_configs.get(0).unwrap()
        )?);

        self.shader = Some(shader);
        self.light_shader = Some(light_shader);

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let ticks = self.ticks();
        let delta = ticks - self.last_frame;
        
        self.last_frame = ticks;
       
        let mut velocity = (0.0, 0.0); // index 0: x, index 1: z
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

                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => velocity.1 += CAMERA_SPEED,
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => velocity.1 -= CAMERA_SPEED,
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => velocity.0 -= CAMERA_SPEED,
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => velocity.0 += CAMERA_SPEED,
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => velocity.1 -= CAMERA_SPEED,
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => velocity.1 += CAMERA_SPEED,
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => velocity.0 += CAMERA_SPEED,
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => velocity.0 -= CAMERA_SPEED,
                Event::MouseMotion { xrel, yrel, .. } => {
                    let sensitivity = 0.1;
                    let euler_angles = s_update_angles(
                        &mut self.registry,
                        &self.camera,
                        xrel as f32 * sensitivity,
                        yrel as f32 * sensitivity,
                        -89.0,
                        89.0
                    ).unwrap();
    
                    s_rotate(
                        &mut self.registry,
                        &self.camera,
                        euler_angles
                    );
                },
                _event => ()
            };
        }

        let camera_velocity = self.registry.get_component_mut::<CVelocity>(&self.camera).unwrap();
        camera_velocity.x += velocity.0;
        camera_velocity.z += velocity.1;
        let velocity = glm::vec3(camera_velocity.x, camera_velocity.y, camera_velocity.z);

        s_apply_velocity(
            &mut self.registry,
            &self.camera,
            delta,
            velocity
        )?;
        let camera_pos = self.registry.get_component::<CPosition>(&self.camera).unwrap();
        let camera_dir = self.registry.get_component::<CGizmo3D>(&self.camera).unwrap().front;

        engine::gfx::buffer::clear_buffer(Some((0.02, 0.02, 0.02, 1.0)));
        
        if self.direction_light_on {
            systems::draw(
                &self.direction_light.unwrap(),
                &self.registry,
                &self.light_shader.unwrap(),
                &self.camera
            )?;
        }
        if self.point_light_on {
            systems::draw(
                &self.point_light.unwrap(),
                &self.registry,
                &self.light_shader.unwrap(),
                &self.camera
            )?;
        }

        let shader = self.registry.get_resource::<Shader>(&self.shader.unwrap()).unwrap().program();
        shader.set_int("dirLightOn", self.direction_light_on as i32);
        shader.set_int("pointLightOn", self.point_light_on as i32);
        shader.set_int("spotLightOn", self.spot_light_on as i32);
        shader.set_float_3("spotLight.position", (camera_pos.x, camera_pos.y, camera_pos.z));
        shader.set_float_3("spotLight.direction", (camera_dir.x, camera_dir.y, camera_dir.z));
        
        for entity in self.crates.iter() {
            systems::update_entity(entity, &self.registry);
            systems::draw(
                entity,
                &self.registry,
                &self.shader.unwrap(),
                &self.camera
            )?;
        }

        Ok(Some(()))
    }
}

extern crate skald;
extern crate nalgebra_glm as glm;

use skald::{
    resources::{
        Shader,
        shader::UniformVariable
    },
    VersionedIndex,
    components::{
        CMaterial,
        material::MaterialPart,
        CGizmo3D,
        CVelocity, CTransform
    },
    systems::{
        movement::s_apply_velocity,
        rotation::{
            s_update_angles,
            s_rotate_camera
        },
        draw::{
            s_draw_by_tag,
            s_draw_entity
        },
        mvp_matrices::*,
        load_obj::{
            ObjectConfig,
            s_load_obj_file
        },
        grid::*
    }, utils::Timer, core::GUI,
};
use sdl2::{
    EventPump,
    keyboard::Keycode,
    event::{
        Event,
        WindowEvent
    }
};

mod systems;
mod config;
mod scene;

use scene::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

type Light = VersionedIndex;

const CAMERA_SPEED: f32 = 5.0;

pub struct MyGame {
    registry: skald::Registry,
    timer: Timer,
    grid: Option<Grid>,
   
    shader: Option<VersionedIndex>,
    light_shader: Option<VersionedIndex>,
    camera: VersionedIndex,
    direction_light: Option<Light>,
    point_light: Option<Light>,
    spot_light: Option<Light>,

    direction_light_on: bool,
    point_light_on: bool,
    spot_light_on: bool,

    _has_control: bool,
    debug_gui: Option<GUI>
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = create_registry()?;
        let timer = Timer::new()?;
        let camera = create_camera(
            &mut registry,
            WIDTH as f32,
            HEIGHT as f32
        )?;

        Ok(Self {
            registry,
            timer,
            grid: None,

            shader: None,
            light_shader: None,
            camera,
            direction_light: None,
            point_light: None,
            spot_light: None,

            direction_light_on: true,
            point_light_on: true,
            spot_light_on: true,
            
            _has_control: true,
            debug_gui: None
        })
    }
}

impl skald::Game for MyGame {
    fn init(&mut self, debug_gui: Option<GUI>) -> Result<(), Box<dyn std::error::Error>> {
        let shader = self.registry.create_resource(
            Shader::new(
                "assets/shaders/lighting",
                vec![
                    UniformVariable::MVPMatrix("mvpMatrix".to_string()),
                    UniformVariable::ModelMatrix("model".to_string()),
                ]
            )?
        )?;
        let light_shader = self.registry.create_resource(
            Shader::new(
                "assets/shaders/light_source",
                vec![
                    UniformVariable::MVPMatrix("mvpMatrix".to_string()),
                    UniformVariable::Color("color".to_string())
                ]
            )?
        )?;
        
        let diffuse = create_texture(
            &mut self.registry,
            "assets/objects/textures/container.png",
        )?;
        let specular = create_texture(
            &mut self.registry,
            "assets/objects/textures/container_specular.png",
        )?;

        create_crates(
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

        let (models_obj, _materials_obj) = s_load_obj_file("assets/objects/sphere.obj".to_string())?;
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
        self.grid = Some(s_create_grid(&mut self.registry)?);
        self.debug_gui = debug_gui;

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let delta = self.timer.delta();
       
        let mut velocity = (0.0, 0.0); // index 0: x, index 1: z
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return Ok(None),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => skald::gfx::view::adjust_viewport_dims(w, h),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
                Event::KeyDown { keycode: Some(Keycode::Num1), repeat: false, .. } => {
                    self.direction_light_on = !self.direction_light_on;
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), repeat: false, .. } => {
                    self.point_light_on = !self.point_light_on;
                },
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

                    s_update_angles(
                        &mut self.registry,
                        &self.camera,
                        xrel as f32 * sensitivity,
                        yrel as f32 * sensitivity,
                        -89.0,
                        89.0
                    );
    
                    s_rotate_camera(
                        &mut self.registry,
                        &self.camera,
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

        // s_set_projection_matrix(&self.camera, &mut self.registry);
        s_set_view_matrix(&self.camera, &mut self.registry);

        let camera_pos = self.registry.get_component::<CTransform>(&self.camera).unwrap().translate;
        let camera_dir = self.registry.get_component::<CGizmo3D>(&self.camera).unwrap().front;

        skald::gfx::gl_clear_buffers(Some((0.02, 0.02, 0.02, 1.0)));
        
        let shader = self.registry.get_resource::<Shader>(&self.shader.unwrap()).unwrap();
        shader.program.set_int("dirLightOn", self.direction_light_on as i32);
        shader.program.set_int("pointLightOn", self.point_light_on as i32);
        shader.program.set_int("spotLightOn", self.spot_light_on as i32);
        shader.program.set_float_3("spotLight.position", (camera_pos.x, camera_pos.y, camera_pos.z));
        shader.program.set_float_3("spotLight.direction", (camera_dir.x, camera_dir.y, camera_dir.z));
        
        s_draw_by_tag(
            "light",
            &self.registry,
            &self.light_shader.unwrap(),
            &self.camera,
            skald::systems::draw::DrawMode::Triangles
        )?;

        systems::update_entities("crate", &self.registry);

        let entities = self.registry.get_entities_by_tag("crate");
        for entity in entities {
            s_draw_entity(
                &entity,
                &self.registry,
                &self.camera,
                shader,
                skald::systems::draw::DrawMode::Triangles
            );
        }

        if let Some(shader) = &self.grid {
            s_draw_grid(&self.registry, &self.camera, shader)?;
        }

        Ok(Some(()))
    }
}

extern crate quipi;
extern crate nalgebra_glm as glm;

use quipi::{
    resources::{
        Shader,
        shader::UniformVariable
    },
    VersionedIndex,
    components::{
        material::MaterialPart,
        CBoundingBox,
        CEulerAngles,
        CGizmo3D,
        CMaterial,
        CTransform,
        CVelocity
    },
    systems::{
        grid::*,
        movement::s_apply_velocity,
        rendering::{
            IRenderer,
            Renderer,
            canvas,
        },
        rotation::{
            s_update_angles,
            s_rotate_camera
        },
        assets::{
            ObjectConfig,
            obj_loader::s_load_obj_file
        }
    },
    utils::Timer,
    wrappers::egui::GUI,
    wrappers::opengl::{
        draw::*,
        capabilities::*,
        buffer::clear_buffers
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

mod systems;
mod config;
mod scene;

use scene::*;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

type Light = VersionedIndex;

const CAMERA_SPEED: f32 = 5.0;

pub struct MyGame {
    registry: quipi::Registry,
    timer: Timer,
    grid: Option<Grid>,
    renderer: Renderer,
   
    shader: Option<VersionedIndex>,
    light_shader: Option<VersionedIndex>,
    // camera: VersionedIndex,
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

        let renderer = Renderer::new(
            &mut registry,
            45.0,
            CBoundingBox {
                right: WIDTH as f32,
                top: HEIGHT as f32,
                near: 0.1,
                far: 100.0,
                ..CBoundingBox::default()
            },
            CTransform {
                translate: glm::vec3(0.0, 1.0, 6.0),
                ..CTransform::default()
            },
            CEulerAngles {
                pitch: 0.0,
                yaw: 90.0,
                roll: 0.0
            }
        )?;

        renderer.update_view_matrix(&mut registry);

        Ok(Self {
            registry,
            timer,
            grid: None,
            renderer,

            shader: None,
            light_shader: None,
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

impl quipi::Game for MyGame {
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
            self.renderer.camera(),
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
        self.grid = Some(Grid::new(&mut self.registry)?);
        self.debug_gui = debug_gui;

        Ok(())
    }

    fn handle_frame(&mut self, event_pump: &mut EventPump) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let delta = self.timer.delta();
        let camera = self.renderer.camera();
       
        let mut velocity = (0.0, 0.0); // index 0: x, index 1: z
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => return Ok(None),
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => canvas::set_dimensions(0, 0, w, h),
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
                        &self.renderer.camera(),
                        xrel as f32 * sensitivity,
                        yrel as f32 * sensitivity,
                        -89.0,
                        89.0
                    );
    
                    s_rotate_camera(
                        &mut self.registry,
                        &self.renderer.camera(),
                    );
                },
                _event => ()
            };
        }

        let camera_velocity = self.registry.get_component_mut::<CVelocity>(&camera).unwrap();
        camera_velocity.x += velocity.0;
        camera_velocity.z += velocity.1;
        let velocity = glm::vec3(camera_velocity.x, camera_velocity.y, camera_velocity.z);

        s_apply_velocity(
            &mut self.registry,
            &camera,
            delta,
            velocity
        )?;

        self.renderer.update_view_matrix(&mut self.registry);

        let camera_pos = self.registry.get_component::<CTransform>(&camera).unwrap().translate;
        let camera_dir = self.registry.get_component::<CGizmo3D>(&camera).unwrap().front;

        clear_buffers((0.02, 0.02, 0.02, 1.0));
        
        let shader = self.registry.get_resource::<Shader>(&self.shader.unwrap()).unwrap();
        shader.program.set_int("dirLightOn", self.direction_light_on as i32);
        shader.program.set_int("pointLightOn", self.point_light_on as i32);
        shader.program.set_int("spotLightOn", self.spot_light_on as i32);
        shader.program.set_float_3("spotLight.position", (camera_pos.x, camera_pos.y, camera_pos.z));
        shader.program.set_float_3("spotLight.direction", (camera_dir.x, camera_dir.y, camera_dir.z));

        gl_enable(GLCapability::AlphaBlending);
        gl_enable(GLCapability::DepthTest);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);
        
        self.renderer.draw_by_tag(
            "light",
            &self.registry,
            &self.light_shader.unwrap(),
            DrawMode::Triangles
        )?;

        systems::update_entities("crate", &self.registry);

        if let Some(shader) = &self.shader {
            let entities = self.registry.get_entities_by_tag("crate");
            for entity in entities {
                self.renderer.draw_entity(
                    &entity,
                    &self.registry,
                    shader,
                    DrawMode::Triangles
                );
            }
        }

        if let Some(grid) = &self.grid {
            grid.draw(&self.registry)?;
        }

        Ok(Some(()))
    }
}

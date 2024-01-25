pub extern crate skald;
pub extern crate nalgebra_glm as glm;

mod systems;
mod ui;

use systems::{
    *,
    update_camera::s_update_camera
};

use skald::{
    Game,
    utils::{
        to_abs_path,
        Timer
    },
    gfx::{
        buffer::clear_buffer,
        ShaderProgram
    },
    Registry,
    resources::{
        Shader,
        register_resources,
        shader::UniformVariable
    },
    components::{
        register_components,
        CEulerAngles,
    },
    VersionedIndex,
    systems::{
        mvp_matrices::*,
        rotation::s_rotate_camera,
        grid::{
            Grid,
            s_create_grid,
            s_draw_grid
        },
        draw::{
            s_draw_by_tag,
            DrawMode
        },
    },
    entity_builders::camera::build_perspective_camera, core::GUI
};
use ui::MyUI;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub struct MyGame {
    registry: Registry,
    timer: Timer,
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,
    grid: Option<Grid>,
    ui: Option<MyUI>,
    debug_gui: Option<GUI>
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;
        let timer = Timer::new()?;

        register_components(&mut registry);
        register_resources(&mut registry);

        let camera = build_perspective_camera(
            &mut registry,
            (5.0, 5.0, 5.0),
            45.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            100.0,
            CEulerAngles {
                pitch: 45.0,
                yaw: -90.0,
                roll: 35.0
            }
        )?;

        s_rotate_camera(&mut registry, &camera);
        s_set_view_matrix(&camera, &mut registry);
        s_set_projection_matrix(&camera, &mut registry);

        Ok(Self {
            registry,
            shader: None,
            camera,
            grid: None,
            ui: None,
            timer,
            debug_gui: None
        })
    }
}

impl Game for MyGame {
    fn init(&mut self, debug_gui: Option<GUI>) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", to_abs_path("assets/shaders/simple")?);
        let shader = ShaderProgram::new(&to_abs_path("assets/shaders/simple")?)?;
        let shader = self.registry.create_resource(Shader {
            program: shader,
            uniforms: vec![
                UniformVariable::MVPMatrix("mvpMatrix".to_string())
            ]
        })?;

        self.grid = Some(s_create_grid(&mut self.registry)?);
        self.shader = Some(shader);
        
        let mut ui = MyUI::init()?;
        ui.create_quad((0.0, 0.0, 0.0, 0.5))?;

        self.ui = Some(ui);
        self.debug_gui = debug_gui;

        scene::s_load_scene(
            &mut self.registry
        )?;

        Ok(())
    }

    fn handle_frame(
        &mut self,
        event_pump: &mut sdl2::EventPump
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let delta = self.timer.delta();

        for event in event_pump.poll_iter() {
            let response = handle_input::s_handle_input(
                &mut self.registry,
                &self.camera,
                event
            )?;
            
            if response.is_none() { return Ok(None) }
        }

        // update camera
        s_update_camera(
            &self.camera,
            &mut self.registry,
            delta
        )?;

        // update debug gui
        if let Some(debug_gui) = &mut self.debug_gui {
            debug_gui.update()?;
        }

        // render
        clear_buffer(Some((0.0, 0.0, 0.0, 1.0)));

        if let Some(shader) = self.shader {
            s_draw_by_tag(
                "cube",
                &self.registry,
                &shader,
                &self.camera,
                DrawMode::Triangles
            )?;
        }

        if let Some(grid) = &self.grid {
            s_draw_grid(&self.registry, &self.camera, grid)?;
        }

        if let Some(ui) = &self.ui {
            ui.draw()?;
        }

        Ok(Some(()))
    }
}

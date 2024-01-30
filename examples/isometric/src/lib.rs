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
    utils::Timer,
    facades::{
        opengl::{
            capabilities::*,
            buffer::clear_buffers,
            draw::DrawMode,
            shader::ShaderProgram,
        },
        egui::GUI,
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
        CTransform, CBoundingBox,
    },
    VersionedIndex,
    systems::{
        rendering::{
            IRenderer,
            Renderer,
        },
        grid::Grid
    },
};
use ui::MyUI;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub struct MyGame {
    registry: Registry,
    timer: Timer,
    shader: Option<VersionedIndex>,
    grid: Option<Grid>,
    ui: Option<MyUI>,
    debug_gui: Option<GUI>,

    renderer3d: Renderer
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;
        let timer = Timer::new()?;

        register_components(&mut registry);
        register_resources(&mut registry);

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
                translate: glm::vec3(5.0, 5.0, 5.0),
                ..CTransform::default()
            },
            CEulerAngles {
                pitch: 45.0,
                yaw: -90.0,
                roll: 35.0
            }
        )?;


        Ok(Self {
            registry,
            shader: None,
            grid: None,
            ui: None,
            timer,
            debug_gui: None,
            renderer3d: renderer
        })
    }
}

impl Game for MyGame {
    fn init(&mut self, debug_gui: Option<GUI>) -> Result<(), Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/simple")?;
        let shader = self.registry.create_resource(Shader {
            program: shader,
            uniforms: vec![
                UniformVariable::MVPMatrix("mvpMatrix".to_string())
            ]
        })?;

        self.grid = Some(Grid::new(&mut self.registry)?);
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
                &self.renderer3d.camera(),
                event
            )?;
            
            if response.is_none() { return Ok(None) }
        }

        // update camera
        s_update_camera(
            &self.renderer3d.camera(),
            &mut self.registry,
            delta
        )?;

        // render
        clear_buffers((0.0, 0.0, 0.0, 1.0));

        gl_enable(GLCapability::DepthTest);
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        if let Some(shader) = self.shader {
            self.renderer3d.draw_by_tag(
                "cube",
                &self.registry,
                &shader,
                DrawMode::Triangles
            )?;
        }

        if let Some(ui) = &self.ui {
            ui.draw()?;
        }

        if let Some(grid) = &self.grid {
            grid.draw(&self.registry)?;
        }

        // update debug gui
        if let Some(debug_gui) = &mut self.debug_gui {
            debug_gui.update()?;
        }

        Ok(Some(()))
    }
}

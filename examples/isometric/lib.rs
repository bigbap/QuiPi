extern crate quipi_3d as quipi;
extern crate nalgebra_glm as glm;

mod systems;
mod ui;

use ui::MyUI;
use systems::{
    *,
    update_camera::s_update_camera
};

use quipi::{
    components::{
        CEulerAngles,
        CTransform,
        CCamera,
        camera::CameraParams
    },
    resources::{
        shader::UniformVariable,
        RShader
    },
    systems::{
        grid::Grid,
        draw::draw_all
    },
    FrameState,
    FrameResponse,
    QuiPiApp,
    GlobalRegistry,
    VersionedIndex
};
use quipi_core::{
    QuiPiWindow,
    wrappers::opengl::{
        capabilities::*,
        buffer::clear_buffers,
        draw::DrawMode,
        shader::ShaderProgram,
    }
};

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub struct MyGame {
    shader: Option<VersionedIndex>,
    grid: Option<Grid>,
    ui: Option<MyUI>,

    camera: Option<VersionedIndex>
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            shader: None,
            grid: None,
            ui: None,
            camera: None
        })
    }
}

impl QuiPiApp for MyGame {
    fn init(
        &mut self,
        registry: &mut GlobalRegistry,
        winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shader = registry.resources.create()?;
        registry.resources.add(&shader, RShader {
            program: ShaderProgram::new("assets/shaders/simple")?,
            uniforms: vec![
                UniformVariable::MVPMatrix("mvpMatrix".to_string())
            ]
        });

        let camera = registry.entity_manager.create()?;
        registry.entity_manager.add(&camera, CCamera::new(CameraParams {
            aspect: WIDTH as f32 / HEIGHT as f32,
            ..CameraParams::default()
        })?);
        registry.entity_manager.add(&camera, CTransform {
            translate: glm::vec3(5.0, 5.0, 5.0),
            ..CTransform::default()
        });
        registry.entity_manager.add(&camera, CEulerAngles {
            pitch: 45.0,
            yaw: -90.0,
            roll: 35.0
        });

        self.grid = Some(Grid::new(registry, camera)?);
        self.shader = Some(shader);
        self.camera = Some(camera);
        
        let mut ui = MyUI::init()?;
        ui.create_quad((0.0, 0.0, 0.0, 0.5))?;

        self.ui = Some(ui);

        scene::s_load_scene(
            registry
        )?;

        Ok(())
    }

    fn handle_frame(
        &mut self,
        registry: &mut GlobalRegistry,
        frame_state: &mut FrameState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        handle_input::s_handle_input(
            frame_state,
            registry,
            &self.camera.unwrap()
        )?;

        // update camera
        s_update_camera(
            &self.camera.unwrap(),
            registry,
            frame_state.delta
        )?;

        // render
        clear_buffers((0.0, 0.0, 0.0, 1.0));

        gl_enable(GLCapability::DepthTest);
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        if let Some(shader) = self.shader {
            draw_all(
                registry,
                DrawMode::Triangles
            )?;
        }

        if let Some(ui) = &self.ui {
            ui.draw()?;
        }

        if let Some(grid) = &self.grid {
            grid.draw(registry, &self.camera.unwrap())?;
        }

        Ok(FrameResponse::None)
    }
}

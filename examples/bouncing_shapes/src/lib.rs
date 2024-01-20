extern crate engine;
extern crate nalgebra_glm as glm;

use std::fs;

use engine::{
    Registry,
    VersionedIndex,
    resources::{Shader, shader::UniformVariable},
    gfx::{
        buffer::clear_buffer,
        ShaderProgram,
    },
    entity_builders::camera::build_ortho_camera,
    math::random::Random,
    utils::{
        now_secs,
        to_abs_path
    },
    systems::{
        draw::s_draw_by_tag,
        mvp_matrices::{
            s_set_ortho_projection_matrix,
            s_set_view_matrix
        }
    }
};

pub static WIDTH: u32 = 800;
pub static HEIGHT: u32 = 600;

mod systems;

use systems::*;

pub struct MyGame {
    registry: Registry,
    timer: std::time::Instant,
    rand: Random,
    
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,

    last_frame: f32
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = engine::Registry::init()?;
        let timer = std::time::Instant::now();
        let rand = Random::from_seed(now_secs()?);

        engine::resources::register_resource(&mut registry);
        engine::components::register_components(&mut registry);

        let camera = build_ortho_camera(
            &mut registry,
            (-0.5, -0.5, 0.0),
            1.0_f32,
            1.0_f32,
            0.0,
            1.0
        )?;

        s_set_ortho_projection_matrix(&camera, &mut registry);
        s_set_view_matrix(&camera, &mut registry);

        Ok(MyGame {
            registry,
            shader: None,
            timer,
            rand,
            camera,
            last_frame: timer.elapsed().as_millis() as f32 / 1000.0
        })
    }

    pub fn ticks(&self) -> f32 {
        self.timer.elapsed().as_millis() as f32 / 1000.0
    }
}

impl engine::Game for MyGame {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new(&format!("{}/shaders/shape", asset_path()))?;
        let shader_id = self.registry.create_resource(Shader {
            program: shader,
            uniforms: vec![
                UniformVariable::MVPMatrix("mvpMatrix".to_string())
            ]
        })?;

        create_shapes(
            &mut self.registry,
            &mut self.rand
        );
        
        self.shader = Some(shader_id);

        Ok(())
    }

    fn handle_frame(
        &mut self,
        event_pump: &mut sdl2::EventPump
    ) -> Result<Option<()>, Box<dyn std::error::Error>> {
        let ticks = self.ticks();
        let delta = ticks - self.last_frame;
        
        self.last_frame = ticks;

        // handle input events
        for event in event_pump.poll_iter() {
            let response = s_handle_input(
                &self.camera,
                &mut self.registry,
                event,
                &mut self.rand
            )?;

            if response.is_none() { return Ok(None) }
        }

        s_update(
            &mut self.registry,
            delta,
        )?;

        // render
        clear_buffer(Some((0.2, 0.2, 0.1, 1.0)));

        s_draw_by_tag(
            "quad",
            &self.registry,
            &self.shader.unwrap(),
            &self.camera,
        )?;

        Ok(Some(()))
    }
}

/**
* Config key
* CIRCLE radius x_pos y_pos r g b
* QUAD width height x_pos y_pos r g b
*/
fn create_shapes(
    registry: &mut Registry,
    rand: &mut Random
) -> Vec<VersionedIndex> {
    let mut shapes: Vec<VersionedIndex> = Vec::new();
    
    for line in fs::read_to_string(format!("{}/config.txt", asset_path())).unwrap().lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let kind = parts.first().unwrap();
        let parts: Vec<f32> = parts[1..]
            .iter()
            .map(|part| part.parse::<f32>().unwrap())
            .collect();

        match *kind {
            "CIRCLE" => s_create_circle(registry, &parts),
            "QUAD" => shapes.push(
                s_create_quad(registry, &parts, rand).unwrap()
            ),
            _ => ()
        }

        // for _ in 0..1000 {
        //     let _ = s_spawn_quad(registry, rand);
        // }
    }

    shapes
}

fn asset_path() -> String {
    to_abs_path("assets").unwrap().to_string_lossy().to_string()
}

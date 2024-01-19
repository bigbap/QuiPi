extern crate engine;
extern crate nalgebra_glm as glm;

use std::fs;

use engine::{
    Registry,
    VersionedIndex,
    resources::Shader,
    gfx::{
        buffer::clear_buffer,
        ShaderProgram,
        draw::draw_ebo,
    },
    components::{
        CMesh,
        CTransform,
        CVelocity, CPosition
    },
    entity_builders::camera::build_camera_3d,
    systems::mvp_matrices::{
        s_model_matrix_3d,
        s_view_matrix_3d,
        s_projection_matrix_3d,
    }, math::random::Random
};

mod systems;

use systems::{s_handle_input, s_update, s_create_circle, s_create_quad};

const MODEL: &str = "model";
const VIEW: &str = "view";
const PROJECTION: &str = "projection";

pub struct MyGame {
    registry: Registry,
    timer: std::time::Instant,
    rand: Random,

    screen_width: f32,
    screen_height: f32,
    
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,

    last_frame: f32
}

impl MyGame {
    pub fn new(width: f32, height: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = engine::Registry::init()?;
        let timer = std::time::Instant::now();
        let rand = Random::from_seed(2331);

        engine::resources::register_resource(&mut registry);
        engine::components::register_components(&mut registry);

        let camera = build_camera_3d(
            &mut registry,
            (0.0, 0.0, 1.0),
            75.0,
            width / height,
            0.1,
            100.0
        )?;

        Ok(MyGame {
            registry,
            shader: None,
            timer,
            rand,
            screen_width: width,
            screen_height: height,
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
        let shader = ShaderProgram::new("./examples/bouncing_shapes/assets/shaders/shape")?;
        let shader_id = self.registry.create_resource(Shader(shader))?;

        create_shapes(
            &mut self.registry,
            self.screen_width,
            self.screen_height,
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

        // let velocity = self.registry.get_component::<CVelocity>(&self.camera).unwrap();
        // let velocity = glm::vec3(velocity.x, velocity.y, velocity.z);
        // s_apply_velocity(
        //     &mut self.registry,
        //     &self.camera,
        //     delta,
        //     velocity
        // )?;

        s_update(&mut self.registry, delta);

        // render
        clear_buffer(Some((0.2, 0.2, 0.1, 1.0)));

        draw(
            &mut self.registry,
            &self.shader.unwrap(),
            &self.camera
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
    screen_width: f32,
    screen_height: f32,
    rand: &mut Random
) -> Vec<VersionedIndex> {
    let mut shapes: Vec<VersionedIndex> = Vec::new();
    
    for line in fs::read_to_string("./examples/bouncing_shapes/config.txt").unwrap().lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let kind = parts.first().unwrap();
        let parts: Vec<f32> = parts[1..]
            .iter()
            .map(|part| part.parse::<f32>().unwrap())
            .collect();

        match *kind {
            "CIRCLE" => s_create_circle(registry, &parts),
            "QUAD" => shapes.push(
                s_create_quad(registry, &parts, screen_width, screen_height, rand).unwrap()
            ),
            _ => ()
        }
    }

    shapes
}

fn draw(
    registry: &mut Registry,
    shader_id: &VersionedIndex,
    camera_id: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    let entities = registry.get_entities_by_tag("quad");

    for entity in entities.iter() {
        let Some(cmp_mesh) = registry.get_component::<CMesh>(entity) else { return Ok(()) };
        let Some(shader) = registry.get_resource::<Shader>(shader_id) else { return Ok(()) };
        let Some(position) = registry.get_component::<CPosition>(entity) else { return Ok(()) };

        let model = s_model_matrix_3d(entity, registry, position);
        let view = s_view_matrix_3d(camera_id, registry);
        let projection = s_projection_matrix_3d(camera_id, registry);

        shader.program().use_program();
        cmp_mesh.mesh.vao.bind();

        shader.program().set_mat4(MODEL, &model);
        shader.program().set_mat4(VIEW, &view);
        shader.program().set_mat4(PROJECTION, &projection);

        draw_ebo(&cmp_mesh.mesh.vao);

        cmp_mesh.mesh.vao.unbind();
    }

    Ok(())
}

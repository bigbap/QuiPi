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
        ElementArrayMesh, utils::normalise_dims
    },
    components::{
        CMesh,
        CTransform,
        CVelocity
    },
    entity_builders::camera::build_camera_3d,
    systems::{
        mvp_matrices::{
            s_model_matrix_3d,
            s_view_matrix_3d,
            s_projection_matrix_3d
        },
        movement::s_apply_velocity
    }
};

mod systems;

use systems::s_handle_input;

const MODEL: &str = "model";
const VIEW: &str = "view";
const PROJECTION: &str = "projection";

pub struct MyGame {
    registry: Registry,
    timer: std::time::Instant,

    screen_width: f32,
    screen_height: f32,
    
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,
    shapes: Vec<VersionedIndex>,

    last_frame: f32
}

impl MyGame {
    pub fn new(width: f32, height: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = engine::Registry::init()?;
        let timer = std::time::Instant::now();

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
            screen_width: width,
            screen_height: height,
            camera,
            shapes: vec![],
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

        self.shapes = create_shapes(
            &mut self.registry,
            self.screen_width,
            self.screen_height
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
                event
            )?;

            if response.is_none() { return Ok(None) }
        }

        let velocity = self.registry.get_component::<CVelocity>(&self.camera).unwrap();
        let velocity = glm::vec3(velocity.x, velocity.y, velocity.z);
        s_apply_velocity(
            &mut self.registry,
            &self.camera,
            delta,
            velocity
        )?;


        // render
        clear_buffer(Some((0.2, 0.2, 0.1, 1.0)));

        for shape in self.shapes.iter() {
            draw(
                shape,
                &mut self.registry,
                &self.shader.unwrap(),
                &self.camera
            )?;
        }

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
    screen_height: f32
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
            "CIRCLE" => create_circle(registry, &parts),
            "QUAD" => shapes.push(
                create_quad(registry, &parts, screen_width, screen_height).unwrap()
            ),
            _ => ()
        }
    }

    shapes
}

fn create_quad(
    registry: &mut Registry,
    parts: &[f32],
    screen_width: f32,
    screen_height: f32
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let [width, height, center_x, center_y, r, g, b] = parts else { todo!() };
    let (width, height) = normalise_dims(*width, *height, screen_width, screen_height);
    let (center_x, center_y) = normalise_dims(*center_x, *center_y, screen_width, screen_height);

    let top_left = (center_x - (width / 2.0), center_y + (height / 2.0));
    let bottom_left = (center_x - (width / 2.0), center_y - (height / 2.0));
    let top_right = (center_x + (width / 2.0), center_y + (height / 2.0));
    let bottom_right = (center_x + (width / 2.0), center_y - (height / 2.0));

    let points: Vec<f32> = vec![
        top_left.0, top_left.1, 0.0, top_right.0, top_right.1, 0.0, bottom_right.0, bottom_right.1, 0.0,
        bottom_left.0, bottom_left.1, 0.0, top_left.0, top_left.1, 0.0, bottom_right.0, bottom_right.1, 0.0
    ];

    println!("{:?}", points);
    let r = *r;
    let g = *g;
    let b = *b;
    let color: Vec<f32> = vec![
        r, g, b, r, g, b, r, g, b,
        r, g, b, r, g, b, r, g, b
    ];
    let indices = vec![
        0, 1, 2,
        3, 0, 2
    ];

    let mesh = ElementArrayMesh::new(&indices)?;
    mesh
        .create_vbo_at(&points, 0, 3)?
        .create_vbo_at(&color, 1, 3)?;

    let quad = registry.create_entity()?
        .with(CMesh { mesh })?
        .with(CTransform {
            translate: Some(glm::vec3(0.0, 0.0, 0.0)),
            scale: Some(glm::vec3(0.5, 0.5, 0.5)),
            ..CTransform::default()
        })?
        .done()?;

    Ok(quad)
}

fn create_circle(
    _registry: &mut Registry,
    parts: &[f32],
) {
    let [radius, center_x, center_y, r, g, b] = parts else { todo!() };

    println!("{radius}, {center_x}, {center_y}, {r}, {g}, {b}");
}

fn draw(
    entity: &VersionedIndex,
    registry: &mut Registry,
    shader_id: &VersionedIndex,
    camera_id: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(cmp_mesh) = registry.get_component::<CMesh>(entity) else { return Ok(()) };
    let Some(shader) = registry.get_resource::<Shader>(shader_id) else { return Ok(()) };

    let model = s_model_matrix_3d(entity, registry);
    let view = s_view_matrix_3d(camera_id, registry);
    let projection = s_projection_matrix_3d(camera_id, registry);

    shader.program().use_program();
    cmp_mesh.mesh.vao.bind();

    shader.program().set_mat4(MODEL, &model);
    shader.program().set_mat4(VIEW, &view);
    shader.program().set_mat4(PROJECTION, &projection);

    draw_ebo(&cmp_mesh.mesh.vao);

    Ok(())
}

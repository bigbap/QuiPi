extern crate engine;
extern crate nalgebra_glm as glm;

use std::fs;

use engine::{
    Registry,
    VersionedIndex,
    resources::{
        Shader,
        Camera3D
    },
    gfx::{
        object_loader::ObjectConfig,
        buffer::clear_buffer,
        ShaderProgram,
        draw::draw_ebo
    },
    components::{
        Mesh,
        Color,
        Draw,
        ModelTransform,
        transform::Transforms
    }
};
use sdl2::{
    event::Event,
    keyboard::Keycode,
};

const COLOR_VARIABLE: &str = "color";
const MODEL: &str = "model";
const VIEW: &str = "view";
const PROJECTION: &str = "projection";
const VIEW_POS: &str = "viewPos";

pub struct MyGame {
    registry: Registry,
    timer: std::time::Instant,
    
    shader: Option<VersionedIndex>,
    camera: VersionedIndex,
    shapes: Vec<VersionedIndex>,

    last_frame: f32
}

impl MyGame {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = engine::Registry::init()?;
        let timer = std::time::Instant::now();

        engine::resources::register_resource(&mut registry);
        engine::components::register_components(&mut registry);

        let camera = registry.create_resource(Camera3D {
            position: glm::vec3(0.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            projection: engine::resources::CameraProjection::Orthographic(800.0, 600.0),
            ..Camera3D::default()
        })?;

        Ok(MyGame {
            registry,
            shader: None,
            timer,
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
        println!("before shader");
        let shader = ShaderProgram::new("./examples/bouncing_shapes/assets/shaders/shape")?;
        println!("after shader");
        let shader_id = self.registry.create_resource(Shader(shader))?;

        self.shapes = create_shapes(
            &mut self.registry,
            shader_id,
            self.camera
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

        let camera = self.registry.get_resource_mut::<Camera3D>(&self.camera).unwrap();

        // handle input events
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => camera.move_up = true,
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => camera.move_down = true,
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => camera.move_left = true,
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => camera.move_right = true,
                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => camera.move_up = false,
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => camera.move_down = false,
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => camera.move_left = false,
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => camera.move_right = false,
                // Event::MouseMotion { xrel, yrel, .. } => {
                //     let sensitivity = 0.1;
                //     camera.rotate(
                //         xrel as f32 * sensitivity,
                //         yrel as f32 * sensitivity
                //     );
                // },
                _ => ()
            }
        }

        camera.apply_move(20.0 * delta);
        
        // render
        clear_buffer(Some((0.2, 0.2, 0.1, 1.0)));

        for shape in self.shapes.iter() {
            draw(shape, &mut self.registry)?;
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
    shader_id: VersionedIndex,
    camera_id: VersionedIndex
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
            "CIRCLE" => create_circle(registry, &parts, shader_id, camera_id),
            "QUAD" => shapes.push(create_quad(registry, &parts, shader_id, camera_id).unwrap()),
            _ => ()
        }
    }

    shapes
}

fn create_quad(
    registry: &mut Registry,
    parts: &[f32],
    shader_id: VersionedIndex,
    camera_id: VersionedIndex
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let [width, height, center_x, center_y, r, g, b] = parts else { todo!() };

    let top_left = (center_x - (width / 2.0), center_y + (height / 2.0));
    let bottom_left = (center_x - (width / 2.0), center_y - (height / 2.0));
    let top_right = (center_x + (width / 2.0), center_y + (height / 2.0));
    let bottom_right = (center_x + (width / 2.0), center_y - (height / 2.0));

    let points: Vec<f32> = vec![
        top_left.0, top_left.1, 0.0, top_right.0, top_right.1, 0.0, bottom_right.0, bottom_right.1, 0.0,
        bottom_left.0, bottom_left.1, 0.0, top_left.0, top_left.1, 0.0, bottom_right.0, bottom_right.1, 0.0
    ];

    println!("{:?}", points);
    let indices = vec![
        0, 1, 2,
        3, 0, 2
    ];

    let obj_config = ObjectConfig {
        points,
        indices,
        ..ObjectConfig::default()
    };

    let quad = registry.create_entity()?
        .with(Mesh::new(&obj_config)?)?
        .with(Color(*r, *g, *b, 1.0))?
        .with(ModelTransform {
            transforms: vec![
                Transforms {
                    translate: Some(glm::vec3(0.0, 0.0, 0.0)),
                    scale: Some(glm::vec3(0.5, 0.5, 0.5)),
                    ..Transforms::default()
                }
            ]
        })?
        .with(Draw {
            shader_id,
            camera_id,
            ..Draw::default()
        })?
        .done()?;

    Ok(quad)
}

fn create_circle(
    registry: &mut Registry,
    parts: &[f32],
    shader_id: VersionedIndex,
    camera_id: VersionedIndex
) {
    let [radius, center_x, center_y, r, g, b] = parts else { todo!() };

    println!("{radius}, {center_x}, {center_y}, {r}, {g}, {b}");
}

fn draw(
    entity: &VersionedIndex,
    registry: &mut Registry,
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(cmp_draw) = registry.get_component::<Draw>(entity) else { return Ok(()) };
    let Some(cmp_mesh) = registry.get_component::<Mesh>(entity) else { return Ok(()) };
    let Some(cmp_transforms) = registry.get_component::<ModelTransform>(entity) else { return Ok(()) };
    let Some(cmp_color) = registry.get_component::<Color>(entity) else { return Ok(()) };
    let Some(shader) = registry.get_resource::<Shader>(&cmp_draw.shader_id) else { return Ok(()) };
    let Some(camera) = registry.get_resource::<Camera3D>(&cmp_draw.camera_id) else { return Ok(()) };

    let models = cmp_transforms.apply_transforms()?;

    shader.program().use_program();
    cmp_mesh.vao().bind();

    for model in models {
        
        shader.program().set_mat4(MODEL, &model);
        shader.program().set_mat4(VIEW, &camera.get_view());
        shader.program().set_mat4(PROJECTION, &camera.get_projection());

        draw_ebo(cmp_mesh.vao());
    }

    Ok(())
}

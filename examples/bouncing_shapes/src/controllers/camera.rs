use quipi_2d::{resources::RCamera2D, schemas::SchemaCamera2D};
use quipi_core::{core::canvas::{get_dimensions, set_dimensions}, FrameResponse, FrameState, IController, Registry};
use sdl2::{event::{Event, WindowEvent}, keyboard::Keycode};

pub const MAIN_CAMERA: &str = "main_camera";

pub struct CameraController {
    camera: u64,
    velocity: glm::Vec2,
    speed: f32
}

impl CameraController {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {

        let Some(camera) = registry.get_resource_id(MAIN_CAMERA) else {
            return Err("[camera controller] camera resource has not been loaded".into());
        };

        Ok(Self {
            camera,
            velocity: glm::vec2(0.0, 0.0),
            speed: 3.0
        })
    }
}

impl IController for CameraController {
    fn update(&mut self, frame_state: &mut FrameState, registry: &mut Registry) -> FrameResponse {
        for event in frame_state.events.iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    set_dimensions(0, 0, *w, *h);

                    if let Some(camera) = registry.get_resource_mut::<RCamera2D>(self.camera) {
                        camera.params.right = *w as f32;
                        camera.params.top = *h as f32;
                    }
                },
                Event::KeyDown { keycode, repeat: false, .. } => {
                    match keycode {
                        Some(Keycode::W) => self.velocity.y += self.speed, // move up
                        Some(Keycode::S) => self.velocity.y -= self.speed, // move down
                        Some(Keycode::A) => self.velocity.x -= self.speed, // move left
                        Some(Keycode::D) => self.velocity.x += self.speed, // move right,
                        _ => ()
                    }
                },
                Event::KeyUp { keycode, repeat: false, .. } => {
                    match keycode {
                        Some(Keycode::W) => self.velocity.y -= self.speed, // move up
                        Some(Keycode::S) => self.velocity.y += self.speed, // move down
                        Some(Keycode::A) => self.velocity.x += self.speed, // move left
                        Some(Keycode::D) => self.velocity.x -= self.speed, // move right,
                        _ => ()
                    }
                },
                _ => ()
            };
        }

        if let Some(camera) = registry.get_resource_mut::<RCamera2D>(self.camera) {
            camera.transform.translate.x += self.velocity.x;
            camera.transform.translate.y += self.velocity.y;
        }
    
        FrameResponse::None
    }
}

pub fn camera_schema() -> SchemaCamera2D {
    let (_x, _y, width, height) = get_dimensions();
    SchemaCamera2D {
        name: MAIN_CAMERA.to_string(),
        right: width as f32,
        top: height as f32,
        ..SchemaCamera2D::default()
    }
}
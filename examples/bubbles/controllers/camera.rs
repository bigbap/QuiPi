use crate::{
    qp_ecs::resources::RCamera2D,
    qp_schemas::SchemaCamera2D,
    qp_gfx::viewport::{
        get_dimensions,
        set_dimensions
    },
    qp_data::{
        FrameResponse,
        FrameState,
        IController,
    },
    Registry
};
use quipi::prelude::QPError;
use sdl2::event::{Event, WindowEvent};

pub const MAIN_CAMERA: &str = "main_camera";

pub struct CameraController {
    camera: u64,
}

impl CameraController {
    pub fn new(registry: &mut Registry) -> Result<Self, QPError> {

        let Some(camera) = registry.get_resource_id(MAIN_CAMERA) else {
            return Err(QPError::Generic("[camera controller] camera resource has not been loaded".to_string()));
        };

        Ok(Self {
            camera,
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
                _ => ()
            };
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
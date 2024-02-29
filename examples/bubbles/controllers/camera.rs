use crate::{
    qp_assets::RCamera2D,
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
    GlobalRegistry
};
use quipi::prelude::QPError;
use sdl2::{event::{Event, WindowEvent}, mouse::MouseWheelDirection};

pub const MAIN_CAMERA: &str = "main_camera";

pub struct CameraController {
    camera: u64,
}

impl CameraController {
    pub fn new(registry: &mut GlobalRegistry) -> Result<Self, QPError> {

        let Some(camera) = registry.asset_manager.get_asset_id(MAIN_CAMERA) else {
            return Err(QPError::Generic("[camera controller] camera resource has not been loaded".to_string()));
        };

        Ok(Self {
            camera,
        })
    }
}

impl IController for CameraController {
    fn update(&mut self, frame_state: &mut FrameState, registry: &mut GlobalRegistry) -> FrameResponse {
        for event in frame_state.events.iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    set_dimensions(0, 0, *w, *h);

                    if let Some(camera) = registry.asset_manager.get_mut::<RCamera2D>(self.camera) {
                        camera.params.right = *w as f32;
                        camera.params.top = *h as f32;
                    }
                },
                Event::MouseWheel { timestamp, window_id, which, x, y, direction, precise_x, precise_y } => {
                    if let Some(camera) = registry.asset_manager.get_mut::<RCamera2D>(self.camera) {
                        camera.set_zoom(*precise_y * frame_state.delta);
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
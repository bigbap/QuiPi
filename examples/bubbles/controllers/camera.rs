use crate::{qp_assets::RCamera2D, qp_schemas::SchemaCamera2D, GlobalRegistry};
use quipi::{
    app::{Controller, FrameResult},
    prelude::QPError,
    world::World,
};
use sdl2::event::{Event, WindowEvent};

pub const MAIN_CAMERA: &str = "main_camera";

pub struct CameraController {
    pub camera: u64,
}

impl CameraController {
    pub fn new(registry: &mut GlobalRegistry) -> Result<Self, QPError> {
        let Some(camera) = registry.asset_manager.get_asset_id(MAIN_CAMERA) else {
            return Err(QPError::Generic(
                "[camera controller] camera resource has not been loaded".to_string(),
            ));
        };

        Ok(Self { camera })
    }
}

impl Controller for CameraController {
    fn update(&mut self, world: &mut World) -> FrameResult {
        for event in world.events.iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    world.viewport.set_dimensions(0, 0, *w, *h);

                    if let Some(camera) = world
                        .registry
                        .asset_manager
                        .get_mut::<RCamera2D>(self.camera)
                    {
                        camera.params.right = *w as f32;
                        camera.params.top = *h as f32;

                        camera.view = camera.calc_view_matrix();
                        camera.projection = camera.calc_projection_matrix();
                    }
                }
                Event::MouseWheel { precise_y, .. } => {
                    if let Some(camera) = world
                        .registry
                        .asset_manager
                        .get_mut::<RCamera2D>(self.camera)
                    {
                        camera.set_zoom(camera.zoom + (*precise_y * world.delta * 7.0));
                    }
                }
                _ => (),
            };
        }

        FrameResult::None
    }
}

pub fn camera_schema(width: f32, height: f32) -> SchemaCamera2D {
    SchemaCamera2D {
        name: MAIN_CAMERA.to_string(),
        right: width,
        top: height,
        ..SchemaCamera2D::default()
    }
}

use crate::{
    qp_assets::RCamera2D, qp_ecs::components::CTransform2D, qp_schemas::SchemaCamera2D,
    GlobalRegistry, VersionedIndex,
};
use quipi::{
    app::{Controller, FrameResult},
    prelude::QPError,
    world::World,
};
use sdl2::event::{Event, WindowEvent};

pub const MAIN_CAMERA: &str = "main_camera";

pub struct CameraController {
    camera: u64,

    player: VersionedIndex, // camera will follow player
}

impl CameraController {
    pub fn new(player: VersionedIndex, registry: &mut GlobalRegistry) -> Result<Self, QPError> {
        let Some(camera) = registry.asset_manager.get_asset_id(MAIN_CAMERA) else {
            return Err(QPError::Generic(
                "[camera controller] camera resource has not been loaded".into(),
            ));
        };

        Ok(Self { camera, player })
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
                    }
                }
                _ => (),
            };
        }

        let mut x = 0.0;
        let mut y = 0.0;
        if let Some(player) = world
            .registry
            .entity_manager
            .get::<CTransform2D>(&self.player)
        {
            x = player.translate.x;
            y = player.translate.y;
        }

        if let Some(camera) = world
            .registry
            .asset_manager
            .get_mut::<RCamera2D>(self.camera)
        {
            camera.transform.translate.x = x - (camera.params.right / 2.0);
            camera.transform.translate.y = y - (camera.params.top / 2.0);

            camera.view = camera.calc_view_matrix();
        }

        FrameResult::None
    }
}

pub fn camera_schema(width: f32, height: f32) -> SchemaCamera2D {
    SchemaCamera2D {
        name: MAIN_CAMERA.to_string(),
        right: width as f32,
        top: height as f32,
        ..SchemaCamera2D::default()
    }
}

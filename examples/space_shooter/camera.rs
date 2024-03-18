use quipi::{
    assets::AssetId,
    common::components::components::{COrthographic, CTransform2D},
    gfx::{
        prelude::quad::QUAD_SHADER_NAME,
        render::{
            cameras::{camera_bundle, matrix_ortho, CMatrix4, CameraMetadata},
            renderers::quads::RenderQuads,
            viewport::Viewport,
        },
    },
    plugin::Plugin,
    prelude::{Component, Index, Res, ResMut, StorageId, StorageManager},
    resources::{AsAny, Resource},
    schedule::Update,
    QPResult,
};

pub struct Camera;
impl Plugin for Camera {
    fn build(&self, app: &mut quipi::prelude::App) -> QPResult<()> {
        let viewport = app.world.resource::<Viewport>().unwrap();
        let viewport = viewport.get_dimensions();
        let interner = app.world.interner_mut();
        let shader_id = AssetId::Id(interner.intern(QUAD_SHADER_NAME));
        let camera_id = app
            .world
            .storage_manager_mut()
            .get_mut(StorageId::Cameras)
            .unwrap()
            .spawn(camera_bundle(
                MainCamera,
                CameraMetadata {
                    width: viewport.width as u32,
                    height: viewport.height as u32,
                    ..Default::default()
                },
            ));
        let camera_state = CameraState {
            this: camera_id,
            player: None,
        };
        app.add_plugins(RenderQuads::new(shader_id, camera_id));
        app.add_system(Update, resize);
        app.add_system(Update, follow);
        app.add_resource(camera_state);

        Ok(())
    }
}

#[derive(Debug, Component, PartialEq, Clone, Copy)]
struct MainCamera;

#[derive(Debug, Resource, AsAny, PartialEq, Clone, Copy)]
struct CameraState {
    player: Option<Index>,
    this: Index,
}

fn resize(storage: ResMut<StorageManager>, viewport: Res<Viewport>, state: Res<CameraState>) {
    let (Some(storage), Some(viewport), Some(state)) = (storage, viewport, state) else {
        return;
    };

    let Some(params) = storage
        .get_mut(StorageId::Cameras)
        .unwrap()
        .get_mut::<COrthographic>(&state.this)
    else {
        return;
    };
    let dims = viewport.get_dimensions();
    params.right = dims.width as f32;
    params.top = dims.height as f32;
}

fn follow(storage: ResMut<StorageManager>, state: Res<CameraState>) {
    let (Some(storage), Some(state)) = (storage, state) else {
        return;
    };

    let Some(player) = state.player else { return };

    let (Some(params), Some(target)) = (
        storage
            .get(StorageId::Cameras)
            .unwrap()
            .get::<COrthographic>(&state.this),
        storage
            .get(StorageId::Entities)
            .unwrap()
            .get::<CTransform2D>(&player),
    ) else {
        return;
    };

    let offset = 50.0;
    let lerp_factor = 0.5;

    let this = state.this;
    let width = params.right;
    let height = params.top;
    let center = glm::vec2(width / 2.0, height / 2.0);
    let offset = target.direction() * offset;
    let target = target.translate - center + offset;

    let Some(transform) = storage
        .get_mut(StorageId::Cameras)
        .unwrap()
        .get_mut::<CTransform2D>(&this)
    else {
        return;
    };
    transform.translate = glm::lerp(&transform.translate, &target, lerp_factor);

    let position = glm::vec3(transform.translate.x, transform.translate.y, 0.0);
    let Some(matrix) = storage
        .get_mut(StorageId::Cameras)
        .unwrap()
        .get_mut::<CMatrix4>(&state.this)
    else {
        return;
    };

    matrix.0 = matrix_ortho(width as u32, height as u32, &position);
}

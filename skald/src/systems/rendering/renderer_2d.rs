use crate::{
    components::*,
    gfx::opengl::draw::DrawMode,
    systems::rendering::{
        draw::s_draw_by_tag,
        draw::s_draw_entity,
        IRenderer,
        matrices::view_matrix,
    },
    Registry,
    VersionedIndex
};

use self::camera::{CameraParams, OrthographicParams};

#[derive(Debug)]
pub struct Renderer2D {
    camera: VersionedIndex,
    camera_params: CameraParams
}

impl IRenderer for Renderer2D {
    fn camera(&self) -> VersionedIndex { self.camera }

    fn update_view_matrix(&self, registry: &mut Registry) {
        if let (Some(_), Some(transform), Some(gizmo)) = (
            registry.get_component::<CViewMatrix>(&self.camera),
            registry.get_component::<CTransform>(&self.camera),
            registry.get_component::<CGizmo3D>(&self.camera),
        ) {
            let matrix = view_matrix(transform, gizmo);

            let v_matrix = registry.get_component_mut::<CViewMatrix>(&self.camera).unwrap();

            v_matrix.0 = matrix;
        }
    }
}

impl Renderer2D {
    pub fn new(
        registry: &mut Registry,
        b_box: CBoundingBox,
        transform: CTransform,
        angles: CEulerAngles
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let camera_params = CameraParams::Orthographic(OrthographicParams {
            left: b_box.left,
            right: b_box.right,
            bottom: b_box.bottom,
            top: b_box.top,
            near: b_box.near,
            far: b_box.far
        });
        let camera = registry.create_entity("camera")?
            .with(CCamera::new(&camera_params)?)?
            .with(b_box)?
            .with(transform)?
            .with(CGizmo3D::default())?
            .with(angles)?
            .with(CVelocity::default())?
            .with(CMouseBtnState::default())?
            .with(CViewMatrix::default())?
            .done()?;

        Ok(Self {
            camera,
            camera_params
        })
    }

    pub fn update_projection_matrix(
        &self,
        registry: &mut Registry,
        left: Option<f32>,
        right: Option<f32>,
        bottom: Option<f32>,
        top: Option<f32>
    ) {
        if let CameraParams::Orthographic(mut params) = &self.camera_params {
            if let Some(left) = left { params.left = left };
            if let Some(right) = right { params.right = right };
            if let Some(bottom) = bottom { params.bottom = bottom };
            if let Some(top) = top { params.top = top };

            if let Some(camera) = registry.get_component_mut::<CCamera>(&self.camera) {
                camera.update_projection_matrix(&self.camera_params);
            }
        }
    }

    pub fn draw_by_tag(
        &self,
        tag: &str,
        registry: &Registry,
        shader: &VersionedIndex,
        mode: DrawMode
    ) -> Result<(), Box<dyn std::error::Error>> {
        s_draw_by_tag(tag, registry, shader, &self.camera, mode)
    }

    pub fn draw_entity(
        &self,
        entity: &VersionedIndex,
        registry: &Registry,
        shader: &VersionedIndex,
        mode: DrawMode
    ) {
        if let Some(shader) = registry.get_resource(shader) {
            s_draw_entity(entity, registry, &self.camera, shader, mode);
        }
    }
}

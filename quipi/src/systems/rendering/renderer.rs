// use crate::{
//     components::*,
//     wrappers::opengl::draw::DrawMode,
//     systems::rendering::{
//         draw::s_draw_by_tag,
//         draw::s_draw_entity,
//         IRenderer,
//         matrices::view_matrix
//     }, Registry, VersionedIndex
// };
//
// #[derive(Debug)]
// pub struct Renderer {
//     camera: VersionedIndex,
// }
//
// impl IRenderer for Renderer {
//     fn camera(&self) -> VersionedIndex { self.camera }
//
//     fn update_view_matrix(&self, registry: &mut Registry) {
//         if let (Some(_), Some(transform), Some(gizmo)) = (
//             registry.get_component::<CViewMatrix>(&self.camera),
//             registry.get_component::<CTransform>(&self.camera),
//             registry.get_component::<CGizmo3D>(&self.camera),
//         ) {
//             let matrix = view_matrix(transform, gizmo);
//
//             let v_matrix = registry.get_component_mut::<CViewMatrix>(&self.camera).unwrap();
//
//             v_matrix.0 = matrix;
//         }
//     }
// }
//
// impl Renderer {
//     pub fn new(
//         registry: &mut Registry,
//         tag: &str,
//         params: CameraParams,
//         transform: CTransform,
//         angles: CEulerAngles
//     ) -> Result<Self, Box<dyn std::error::Error>> {
//         let camera = registry.create_entity(tag)?
//             .with(CCamera::new(params)?)?
//             .with(transform)?
//             .with(CGizmo3D::default())?
//             .with(angles)?
//             .with(CVelocity::default())?
//             .with(CMouseBtnState::default())?
//             .with(CViewMatrix::default())?
//             .done()?;
//
//         let renderer = Self { camera };
//
//         renderer.update_view_matrix(registry);
//
//         Ok(renderer)
//     }
//
//     pub fn update_projection_matrix(
//         &mut self,
//         registry: &mut Registry,
//         fov: Option<f32>,
//         aspect: Option<f32>,
//     ) {
//         if let Some(camera) = registry.get_component_mut::<CCamera>(&self.camera) {
//             if camera.params.kind != CameraType::Cam3D {
//                 println!("camera is no 3D");
//                 return;
//             }
//
//             if let Some(fov) = fov { camera.params.fov = fov };
//             if let Some(aspect) = aspect { camera.params.aspect = aspect };
//
//             camera.projection = CCamera::calc_projection_matrix(&camera.params);
//         }
//     }
//
//     pub fn draw_by_tag(
//         &self,
//         tag: &str,
//         registry: &Registry,
//         shader: &VersionedIndex,
//         mode: DrawMode
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         s_draw_by_tag(tag, registry, shader, &self.camera, mode)
//     }
//
//     pub fn draw_entity(
//         &self,
//         entity: &VersionedIndex,
//         registry: &Registry,
//         shader: &VersionedIndex,
//         mode: DrawMode
//     ) {
//         if let Some(shader) = registry.get_resource(shader) {
//             s_draw_entity(entity, registry, &self.camera, shader, mode);
//         }
//     }
// }

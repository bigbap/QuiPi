// use quipi_core::{
//     opengl::{
//         self,
//         capabilities::{
//             gl_blending_func,
//             gl_enable,
//             GLBlendingFactor,
//             GLCapability
//         },
//         draw::{DrawBuffer, DrawMode}
//     },
//     resources::RTexture
// };

// use crate::{
//     components::{
//         CCamera2D, CMesh2D, CModelMatrix2D, CSprite, CViewMatrix2D, CRGBA
//     },
//     resources::shader::{
//         RShader,
//         UniformVariable
//     },
//     Registry,
//     VersionedIndex
// };

// /**
// * draw all drawable entities
// *
// * camera must have the following components:
// * - CViewMatrix
// * - CProjectionMatrix
// *
// * entities must have the following components:
// * - CMesh
// * - CDrawable
// *
// * shader must exist and be valid
// */
// pub fn draw_all(
//     registry: &mut Registry,
//     mode: DrawMode
// ) -> Result<(), Box<dyn std::error::Error>> {
//     gl_enable(GLCapability::AlphaBlending);
//     gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

//     let entities = registry.entities.query_all::<CSprite>();

//     for entity in entities.iter() {
//         if let Some(drawable) = registry.entities.get::<CSprite>(entity) {
//             if !drawable.active { continue; }

//             let camera = drawable.camera;
//             let shader = drawable.shader;
//             draw_entity(
//                 entity,
//                 registry,
//                 &camera,
//                 &shader,
//                 mode
//             );
//         }
//     }

//     Ok(())
// }

// pub fn draw_entity(
//     entity: &VersionedIndex,
//     registry: &mut Registry,
//     camera: &VersionedIndex,
//     shader: &VersionedIndex,
//     mode: DrawMode
// ) {
//     CModelMatrix2D::update_model_matrix(entity, registry);

//     if let Some(shader) = registry.resources.get::<RShader>(shader) {
//         bind_textures(entity, registry, shader);

//         if let Some(mesh) = registry.entities.get::<CMesh2D>(entity) {
//             set_uniforms(
//                 entity,
//                 registry,
//                 shader,
//                 camera
//             );

//             draw_node(mesh, shader, mode);
//         }
//     }
// }

// fn draw_node(
//     node: &CMesh2D,
//     shader: &RShader,
//     mode: DrawMode
// ) {
//     if let Some(mesh) = &node.mesh {
//         shader.program.use_program();
//         mesh.vao.bind();
//         opengl::draw::gl_draw(
//             match mesh.ebo {
//                 Some(_) => DrawBuffer::Elements,
//                 _ => DrawBuffer::Arrays
//             },
//             mode,
//             mesh.vao.count()
//         );
//         mesh.vao.unbind();
//     }
// }

// fn set_uniforms(
//     entity: &VersionedIndex,
//     registry: &Registry,
//     shader: &RShader,
//     camera: &VersionedIndex,
// ) {
//     shader.program.use_program();
//     for uniform in shader.uniforms.iter() {
//         match uniform {
//             UniformVariable::Color(var) => set_color(entity, registry, shader, var),
//             UniformVariable::MVPMatrix(var) => {
//                 if let (Some(model), Some(view), Some(c_camera)) = (
//                     registry.entities.get::<CModelMatrix2D>(entity),
//                     registry.entities.get::<CViewMatrix2D>(camera),
//                     registry.entities.get::<CCamera2D>(camera),
//                 ) {
//                     let mvp_matrix = c_camera.projection * view.0 * model.0;

//                     shader.program.set_mat4(var, &mvp_matrix);
//                 }
//             },
//             UniformVariable::ModelMatrix(var) => {
//                 if let Some(model) = registry.entities.get::<CModelMatrix2D>(entity) {
//                     shader.program.set_mat4(var, &model.0)
//                 }
//             },
//             UniformVariable::ViewMatrix(var) => {
//                 if let Some(view) = registry.entities.get::<CViewMatrix2D>(camera) {
//                     shader.program.set_mat4(var, &view.0)
//                 }
//             },
//             UniformVariable::ProjectionMatrix(var) => {
//                 if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
//                     shader.program.set_mat4(var, &c_camera.projection)
//                 }
//             },
//             UniformVariable::NearPlane(var) => {
//                 if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
//                     shader.program.set_float(var, c_camera.params.near)
//                 }
//             },
//             UniformVariable::FarPlane(var) => {
//                 if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
//                     shader.program.set_float(var, c_camera.params.far)
//                 }
//             },
//         }
//     }
// }

// fn bind_textures(
//     entity: &VersionedIndex,
//     registry: &Registry,
//     shader: &RShader
// ) {
//     if let Some(sprite) = registry.entities.get::<CSprite>(entity) {
//         if let Some(texture_id) = sprite.texture {
//             shader.program.use_program();
//             if let Some(texture) = registry.resources.get::<RTexture>(&texture_id) {
//                 shader.program.set_int("u_texture", 0);
//                 texture.0.use_texture(0);
//             }
//         }
//     }
// }

// fn set_color(
//     entity: &VersionedIndex,
//     registry: &Registry,
//     shader: &RShader,
//     var: &str
// ) {
//     if let Some(color) = registry.entities.get::<CRGBA>(entity) {
//         shader.program.set_float_3(var, (color.value[0], color.value[1], color.value[2]));
//     }
// }

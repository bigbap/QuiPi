use skald::{
    VersionedIndex,
    Registry,
    resources::{
        register_resources,
        Shader,
        shader::UniformVariable
    },
    components::{
        CEulerAngles,
        register_components,
        CZPlanes,
        CQuadConfig,
        CDimensions,
        CModelNode,
        CTransform, CModelMatrix
    },
    entity_builders::camera::build_ortho_camera,
    gfx::ElementArrayMesh,
    utils::to_abs_path,
    systems::{
        draw::{
            s_draw_by_tag,
            DrawMode
        },
        mvp_matrices::{
            s_set_model_matrix,
            s_set_view_matrix,
        }
    }
};

use crate::{WIDTH, HEIGHT};

pub struct MyUI {
    registry: Registry,
    shader: VersionedIndex,
    camera: VersionedIndex
}

impl MyUI {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = Registry::init()?;

        register_resources(&mut registry);
        register_components(&mut registry);

        let shader = registry.create_resource(Shader::new(
            &to_abs_path("assets/shaders/ui")?,
            vec![UniformVariable::MVPMatrix("mvpMatrix".to_string())]
        )?)?;

        let camera = build_ortho_camera(
            &mut registry,
            WIDTH as f32,
            HEIGHT as f32,
            CZPlanes { near_plane: 0.0, far_plane: 0.2 },
            CTransform::default(),
            CEulerAngles {
                pitch: 0.0,
                yaw: 0.0,
                roll: 0.0
            }
        )?;
        s_set_view_matrix(&camera, &mut registry);

        Ok(Self {
            registry,
            shader,
            camera,
        })
    }

    pub fn create_quad(
        &mut self,
        color: (f32, f32, f32, f32)
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(dims) = self.registry.get_component::<CDimensions>(&self.camera) {
            let quad_config = CQuadConfig {
                width: 300.0,
                height: dims.height,
                center_x: 0.0,
                center_y: 0.0
            };
            
            let obj_config = quad_config.to_obj_config(color);

            let mesh = ElementArrayMesh::new(&obj_config.indices)?;
            mesh
                .create_vbo_at(&obj_config.points, 0, 3)?
                .create_vbo_at(&obj_config.colors, 1, 4)?;

            let pos = (
                dims.width - (quad_config.width / 2.0),
                dims.height / 2.0,
                0.0
            );
            let quad = self.registry.create_entity("quad")?
                .with(CModelNode { mesh: Some(mesh), ..CModelNode::default() })?
                .with(CTransform {
                    translate: glm::vec3(pos.0, pos.1, pos.2),
                    ..CTransform::default()
                })?
                .with(CDimensions {
                    width: quad_config.width,
                    height: quad_config.height,
                    ..CDimensions::default()
                })?
                .with(CModelMatrix::default())?
                .done()?;

            s_set_model_matrix(&quad, &mut self.registry);
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        s_draw_by_tag(
            "quad",
            &self.registry,
            &self.shader,
            &self.camera,
            DrawMode::Triangles
        )?;

        Ok(())
    }
}

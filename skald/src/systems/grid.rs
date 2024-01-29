use crate::{
    Registry,
    components::{
        CBoundingBox,
        CEulerAngles,
        CModelMatrix,
        CModelNode,
        CTransform
    },
    gfx::{
        ElementArrayMesh,
        mesh::{
            BufferUsage,
            ShaderLocation
        },
        opengl::draw::DrawMode,
        canvas,
    },
    VersionedIndex,
    resources::{
        Shader,
        shader::UniformVariable
    },
    utils::to_abs_path,
    systems::rendering::Renderer,
};

const GRID_TAG: &str = "skald_grid_74872346";

pub struct Grid {
    renderer: Renderer,
    shader: VersionedIndex
}

impl Grid {
    pub fn new(
        registry: &mut Registry
    ) -> Result<Self, Box<dyn std::error::Error>>{
        let indices = &[0, 1, 2, 2, 3, 0];
        let vertices = &[
            -1.0, -1.0, 0.0,
            1.0, -1.0, 0.0,
            1.0, 1.0, 0.0,
            -1.0, 1.0, 0.0,
        ];

        let mut mesh = ElementArrayMesh::new(6, BufferUsage::StaticDraw)?;
        mesh
            .with_ebo(indices)?
            .with_vbo::<3, f32>(
                ShaderLocation::Zero,
                vertices
            )?;

        build_axis(registry, mesh, glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, 0.0))?;

        let shader = registry.create_resource(Shader::new(
            &to_abs_path("assets/shaders/grid")?,
            vec![
                UniformVariable::ProjectionMatrix("projection".to_string()),
                UniformVariable::ViewMatrix("view".to_string()),
                UniformVariable::NearPlane("near".to_string()),
                UniformVariable::FarPlane("far".to_string())
            ]
        )?)?;

        let (_x, _y, width, height) = canvas::get_dimensions();
        Ok(Self {
            renderer: Renderer::new(
                registry,
                width as f32 / height as f32,
                CBoundingBox {
                    right: width as f32,
                    top: height as f32,
                    near: 0.1,
                    far: 100.0,
                    ..CBoundingBox::default()
                },
                CTransform::default(),
                CEulerAngles::default()
            )?,
            shader
        })
    }

    pub fn draw(
        &self,
        registry: &Registry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grid = registry.get_entities_by_tag(GRID_TAG);

        for line in grid {
            self.renderer.draw_entity(
                &line,
                registry,
                &self.shader,
                DrawMode::Triangles
            );
        }

        Ok(())
    }
}

fn build_axis(
    registry: &mut Registry,
    mesh: ElementArrayMesh,
    translate: glm::Vec3,
    scale: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    let transform = CTransform {
        translate,
        scale: Some(scale),
        ..CTransform::default()
    };
    let model_matrix = CModelMatrix(transform.to_matrix());
    registry.create_entity(GRID_TAG)?
        .with(CModelNode {
            mesh: Some(mesh),
            ..CModelNode::default()
        })?
        .with(transform)?
        .with(model_matrix)?
        .done()?;

    Ok(())
}


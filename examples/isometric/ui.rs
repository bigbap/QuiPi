use quipi::{
    VersionedIndex,
    GlobalRegistry,
    resources::{
        register_resources,
        Shader,
        shader::UniformVariable
    },
    components::{
        CEulerAngles,
        register_components,
        CQuadConfig,
        CModelNode,
        CTransform, CModelMatrix, CBoundingBox
    },
    wrappers::opengl::{
        draw::DrawMode,
        buffer::BufferUsage,
    },
    utils::to_abs_path,
    systems::{
        rendering::{
            Renderer2D,
            IRenderer,
            mesh::{
                ElementArrayMesh,
                ShaderLocation
            },
        },
        mvp_matrices::s_set_model_matrix
    }
};

use crate::{WIDTH, HEIGHT};

pub struct MyUI {
    registry: GlobalRegistry,
    shader: VersionedIndex,
    renderer: Renderer2D
}

impl MyUI {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = GlobalRegistry::init()?;

        register_resources(&mut registry);
        register_components(&mut registry);

        let shader = registry.create_resource(Shader::new(
            &to_abs_path("assets/shaders/ui")?,
            vec![UniformVariable::MVPMatrix("mvpMatrix".to_string())]
        )?)?;

        let renderer = Renderer2D::new(
            &mut registry,
            CBoundingBox {
                right: WIDTH as f32,
                top: HEIGHT as f32,
                far: 0.2,
                ..CBoundingBox::default()
            },
            CTransform::default(),
            CEulerAngles::default()
        )?;

        Ok(Self {
            registry,
            shader,
            renderer
        })
    }

    pub fn create_quad(
        &mut self,
        color: (f32, f32, f32, f32)
    ) -> Result<(), Box<dyn std::error::Error>> {
        let camera = self.renderer.camera();

        if let Some(b_box) = self.registry.get_component::<CBoundingBox>(&camera) {
            let quad_config = CQuadConfig {
                width: 300.0,
                height: b_box.height(),
                center_x: 0.0,
                center_y: 0.0
            };
            
            let obj_config = quad_config.to_obj_config(color);

            let mut mesh = ElementArrayMesh::new(
                obj_config.indices.len(),
                BufferUsage::StaticDraw
            )?;
            mesh
                .with_ebo(&obj_config.indices)?
                .with_vbo::<3, f32>(ShaderLocation::Zero, &obj_config.points)?
                .with_vbo::<4, f32>(ShaderLocation::One, &obj_config.colors)?;

            let pos = (
                b_box.width() - (quad_config.width / 2.0),
                b_box.height() / 2.0,
                0.0
            );
            let quad = self.registry.create_entity("quad")?
                .with(CModelNode { mesh: Some(mesh), ..CModelNode::default() })?
                .with(CTransform {
                    translate: glm::vec3(pos.0, pos.1, pos.2),
                    ..CTransform::default()
                })?
                .with(CBoundingBox {
                    right: quad_config.width,
                    top: quad_config.height,
                    ..CBoundingBox::default()
                })?
                .with(CModelMatrix::default())?
                .done()?;

            s_set_model_matrix(&quad, &mut self.registry);
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.renderer.draw_by_tag(
            "quad",
            &self.registry,
            &self.shader,
            DrawMode::Triangles
        )?;

        Ok(())
    }
}

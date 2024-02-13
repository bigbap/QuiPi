use quipi_core::{
    components::{CTag, CTexture, CRGBA},
    opengl::{
        self, capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability}, draw::DrawBuffer
    },
    rendering::{IRenderer, RenderInfo},
    resources::{
        shader::UniformVariable,
        RShader,
        RTexture
    }, utils::Timer, Registry, VersionedIndex
};

use crate::components::{CCamera2D, CMesh, CModelMatrix2D, CDrawable, CViewMatrix2D};

pub struct Renderer2D {
    timer: Timer,
    rendering: bool,
    to_draw: Vec<VersionedIndex>
}

impl Renderer2D {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            rendering: false,
            to_draw: vec![]
        }
    }
}

impl IRenderer for Renderer2D {
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.rendering == true {
            return Err("renderer was not flushed in the last frame".into());
        }

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        self.rendering = true;

        Ok(())
    }

    fn batch_render(
        &mut self,
        _tag: CTag,
        _registry: &mut Registry
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rendering {
            return Err("rendering hasn't been started for frame".into());
        }

        Ok(())
    }

    fn instance_render(
        &mut self,
        _tag: CTag,
        _registry: &mut Registry
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rendering {
            return Err("rendering hasn't been started for frame".into());
        }

        Ok(())
    }

    fn single_render(
        &mut self,
        entity: VersionedIndex,
        registry: &mut Registry
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rendering {
            return Err("rendering hasn't been started for frame".into());
        }

        let (Some(drawable), Some(mesh)) = (
            registry.entities.get::<CDrawable>(&entity),
            registry.entities.get::<CMesh>(&entity),
        ) else { return Ok(()) };

        if !mesh.should_draw { return Ok(()) }
        if registry.resources.get::<RShader>(&drawable.shader).is_none() { return Ok(()) };

        CModelMatrix2D::update_model_matrix(&entity, registry);

        self.to_draw.push(entity);

        Ok(())
    }

    fn flush(&mut self, registry: &Registry) -> RenderInfo {
        self.timer.delta();

        let mut draw_calls = 0;
        while let Some(entity) = self.to_draw.pop() {
            // it's safe to unwrap here because the check is already preformed
            let drawable = registry.entities.get::<CDrawable>(&entity).unwrap();
            let mesh = registry.entities.get::<CMesh>(&entity).unwrap();
            let shader = registry.resources.get::<RShader>(&drawable.shader).unwrap();

            let mode = mesh.draw_mode;

            shader.program.use_program();

            bind_textures(&entity, registry, shader);
            set_uniforms(
                &entity,
                registry,
                shader,
                &drawable.camera
            );

            mesh.mesh.vao.bind();
            opengl::draw::gl_draw(
                match mesh.mesh.ebo {
                    Some(_) => DrawBuffer::Elements,
                    _ => DrawBuffer::Arrays
                },
                mode,
                mesh.mesh.vao.count()
            );
            mesh.mesh.vao.unbind();

            draw_calls += 1;
        }

        self.rendering = false;

        RenderInfo {
            num_draw_calls: draw_calls,
            total_ms: self.timer.delta() * 1000.0
        }
    }
}

fn set_uniforms(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader,
    camera: &VersionedIndex,
) {
    for uniform in shader.uniforms.iter() {
        match uniform {
            UniformVariable::Color(var) => set_color(entity, registry, shader, var),
            UniformVariable::MVPMatrix(var) => {
                if let (Some(model), Some(view), Some(c_camera)) = (
                    registry.entities.get::<CModelMatrix2D>(entity),
                    registry.entities.get::<CViewMatrix2D>(camera),
                    registry.entities.get::<CCamera2D>(camera),
                ) {
                    let mvp_matrix = c_camera.projection * view.0 * model.0;

                    shader.program.set_mat4(var, &mvp_matrix);
                }
            },
            UniformVariable::ModelMatrix(var) => {
                if let Some(model) = registry.entities.get::<CModelMatrix2D>(entity) {
                    shader.program.set_mat4(var, &model.0)
                }
            },
            UniformVariable::ViewMatrix(var) => {
                if let Some(view) = registry.entities.get::<CViewMatrix2D>(camera) {
                    shader.program.set_mat4(var, &view.0)
                }
            },
            UniformVariable::ProjectionMatrix(var) => {
                if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
                    shader.program.set_mat4(var, &c_camera.projection)
                }
            },
            UniformVariable::NearPlane(var) => {
                if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
                    shader.program.set_float(var, c_camera.params.near)
                }
            },
            UniformVariable::FarPlane(var) => {
                if let Some(c_camera) = registry.entities.get::<CCamera2D>(camera) {
                    shader.program.set_float(var, c_camera.params.far)
                }
            },
        }
    }
}

fn bind_textures(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader
) {
    if let Some(texture_id) = registry.entities.get::<CTexture>(entity) {
        if let Some(texture) = registry.resources.get::<RTexture>(&texture_id.0) {
            texture.0.use_texture(0);
            shader.program.set_int("u_texture", 0);
        }
    }
}

fn set_color(
    entity: &VersionedIndex,
    registry: &Registry,
    shader: &RShader,
    var: &str
) {
    if let Some(color) = registry.entities.get::<CRGBA>(entity) {
        shader.program.set_float_3(var, (color.value[0], color.value[1], color.value[2]));
    }
}
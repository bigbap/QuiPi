use quipi_core::{
    components::{CElementArray, CTag, CTexture, CRGBA}, ec_store::EMQuery, opengl::{
        self, capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability}, draw::{DrawBuffer, DrawMode}
    }, rendering::{IRenderer, RenderInfo}, resources::{
        shader::UniformVariable,
        RShader,
        RTexture
    }, utils::Timer, Registry, VersionedIndex
};

use crate::components::{CCamera2D, CDrawable, CModelMatrix2D, CRect, CViewMatrix2D};

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

        self.timer.delta();

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        self.rendering = true;

        Ok(())
    }

    fn batch_render(
        &mut self,
        _tag: CTag,
        registry: &mut Registry
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.rendering {
            return Err("rendering hasn't been started for frame".into());
        }

        let mut indices = Vec::<u32>::new();
        let entities = EMQuery::<CTag, CRect>::query_all(&registry);
        for entity in entities.iter() {
            let rect = registry.entities.get::<CRect>(entity).unwrap();
            let color = registry.entities.get::<CRGBA>(entity);
            let mesh_data = rect.to_mesh(color.cloned());

            let offset = indices.len() as u32;
            for index in mesh_data.indices {
                indices.push(index + offset);
            }

            // self.single_render(*entity, registry)?;
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

        let (Some(drawable), Some(_)) = (
            registry.entities.get::<CDrawable>(&entity),
            registry.entities.get::<CElementArray>(&entity),
        ) else { return Ok(()) };

        if registry.resources.get::<RShader>(&drawable.shader).is_none() { return Ok(()) };

        CModelMatrix2D::update_model_matrix(&entity, registry);

        self.to_draw.push(entity);

        Ok(())
    }

    fn flush(&mut self, registry: &Registry) -> RenderInfo {
        let mut draw_calls = 0;
        while let Some(entity) = self.to_draw.pop() {
            let Some(drawable) = registry.entities.get::<CDrawable>(&entity) else { continue; };
            let Some(buffer_obj) = registry.entities.get::<CElementArray>(&entity) else { continue; };
            let Some(shader) = registry.resources.get::<RShader>(&drawable.shader) else { continue; };

            let buffer_obj = &buffer_obj.0;

            shader.program.use_program();

            bind_textures(&entity, registry, shader);
            set_uniforms(
                &entity,
                registry,
                shader,
                &drawable.camera
            );

            buffer_obj.vao.bind();
            opengl::draw::gl_draw(
                match buffer_obj.ebo {
                    Some(_) => DrawBuffer::Elements,
                    _ => DrawBuffer::Arrays
                },
                DrawMode::Triangles, // TODO: this is hardcoded
                buffer_obj.vao.count()
            );
            buffer_obj.vao.unbind();

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
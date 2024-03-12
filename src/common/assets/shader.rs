use crate::common::resources::{Asset, AssetLoader, Source};
use crate::platform::opengl::shader::ShaderProgram;
use crate::prelude::QPError;
use crate::QPResult;

#[derive(Debug)]
pub struct ShaderAsset {
    pub program: ShaderProgram,
    pub uniforms: Vec<Uniform>,
}

impl Asset for ShaderAsset {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Uniform {
    pub name: &'static str,
    pub kind: UniformKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UniformKind {
    F32,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    Sampler2D,
}

pub struct ShaderLoader {
    pub uniforms: Vec<Uniform>,
    pub source: Source,
}

impl AssetLoader<ShaderAsset> for ShaderLoader {
    fn load(&mut self) -> QPResult<ShaderAsset> {
        let program = match self.source {
            Source::Path(name) => ShaderProgram::from_file(name)?,
            Source::Strings((vert, frag)) => ShaderProgram::from_str(vert, frag)?,
            _ => return Err(QPError::Generic("invalid source for shader".into())),
        };

        Ok(ShaderAsset {
            program,
            uniforms: self.uniforms.clone(),
        })
    }
}
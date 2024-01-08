use engine::Component;
use engine::gfx::Shader as ShaderProgram;

#[derive(Debug, Component)]
pub struct Shader (pub ShaderProgram);

impl Shader {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(
            Self(ShaderProgram::new(file_name)?)
        )
    }

    pub fn program(&self) -> &ShaderProgram {
        &self.0
    }
}

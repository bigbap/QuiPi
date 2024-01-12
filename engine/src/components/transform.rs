use crate::Component;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Transforms {
    pub translate: Option<glm::Vec3>,
    pub rotate: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,

    pub angle: f32
}

#[derive(Debug, Default, Component, PartialEq, Clone)]
pub struct TransformComponent {
    pub transforms: Vec<Transforms>
}

impl TransformComponent {
    pub fn apply_transforms(&self) -> Result<Vec<glm::Mat4>, Box<dyn std::error::Error>> {
        Ok(
            self.transforms.iter().map(|transform| {
                let model = glm::Mat4::identity();
                let model = match transform.translate {
                    Some(translate) => glm::translate(&model, &translate),
                    None => model
                };
                let model = match transform.rotate {
                    None => model,
                    Some(rotate) => glm::rotate(&model, transform.angle, &glm::normalize(&rotate))
                };
                match transform.scale {
                    Some(scale) => glm::scale(&model, &scale),
                    None => model
                }
            }).collect()
        )
    }
}

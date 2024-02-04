use quipi::{
    VersionedIndex,
    schema::SchemaRect,
    Registry,
    math::random::Random, utils::now_secs, components::CTransform,
};

use crate::{WIDTH, HEIGHT};

pub struct RectSpawner {
    shader: VersionedIndex,
    schema: SchemaRect,
    rand: Random
}

impl RectSpawner {
    pub fn new(
        shader: &VersionedIndex,
    ) -> Result<RectSpawner, Box<dyn std::error::Error>> {
        let schema = SchemaRect {
            transform: CTransform {
                translate: glm::vec3(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5, 0.0),
                scale: Some(glm::vec3(0.2, 0.2, 0.0)),
                ..CTransform::default()
            },
            ..SchemaRect::default()
        };

        Ok(Self {
            shader: *shader,
            schema,
            rand: Random::from_seed(now_secs()?)
        })
    }

    pub fn spawn(
        &mut self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let mut this_schema = self.schema.clone();

        let mut vel = (
            self.rand.range(0, 200) as f32,
            self.rand.range(0, 200) as f32,
        );
        if self.rand.random() > 0.5 { vel.0 *= -1.0; }
        if self.rand.random() > 0.5 { vel.1 *= -1.0; }

        this_schema.velocity.x = vel.0;
        this_schema.velocity.y = vel.1;
        this_schema.color = glm::vec4(
            self.rand.random(),
            self.rand.random(),
            self.rand.random(),
            0.5
        );

        this_schema.build_rect(registry, &self.shader)
    }
}

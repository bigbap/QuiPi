use quipi::{
    VersionedIndex,
    schemas::{
        SchemaScene,
        rect::SchemaRectInstance, IPrefab
    },
    Registry,
    math::random::Random,
    utils::now_secs, components::{CRGBA, CTransform, CBoundingBox, CVelocity},
};

pub struct RectSpawner {
    camera: VersionedIndex,
    rand: Random
}

impl RectSpawner {
    pub fn new(camera: VersionedIndex) -> Result<RectSpawner, Box<dyn std::error::Error>> {
        Ok(Self {
            camera,
            rand: Random::from_seed(now_secs()?)
        })
    }

    pub fn spawn(
        &mut self,
        scene: &mut SchemaScene,
        registry: &mut Registry,
    ) -> Result<Option<VersionedIndex>, Box<dyn std::error::Error>> {
        let Some(b_box) = registry.entities.get::<CBoundingBox>(&self.camera) else {
            return Ok(None);
        };

        let mut vel = (
            self.rand.range(0, 200) as f32,
            self.rand.range(0, 200) as f32,
        );
        if self.rand.random() > 0.5 { vel.0 *= -1.0; }
        if self.rand.random() > 0.5 { vel.1 *= -1.0; }

        let velocity = CVelocity { x: vel.0, y: vel.1, z: 0.0 };
        let color = CRGBA {
            r: self.rand.random(),
            g: self.rand.random(),
            b: self.rand.random(),
            a: 0.5
        };

        let s_factor = self.rand.range(25, 50) as f32 / 100.0;
        let instance = SchemaRectInstance {
            transform: CTransform {
                translate: glm::vec3(
                    b_box.right / 2.0,
                    b_box.top / 2.0,
                    0.0
                ),
                scale: Some(glm::vec3(s_factor, s_factor, s_factor)),
                ..CTransform::default()
            },
            color,
            velocity,
        };

        let Some(this_schema) = scene.rects.get_mut(0) else { return Ok(None) };
        let id = this_schema.build_instance(registry, &instance)?;
        this_schema.instances.push(instance);

        Ok(Some(id))
    }
}

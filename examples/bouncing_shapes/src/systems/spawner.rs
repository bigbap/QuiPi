use quipi::{
    components::{
        CBoundingBox2D, CQuad, CTransform2D, CVelocity2D
    },
    schemas::{
        ISchema, SchemaEntity2D
    },
    Registry,
    VersionedIndex
};
use quipi_core::{components::{CName, CTag}, math::random::Random, utils::now_secs};

use crate::{HEIGHT, WIDTH};

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
        registry: &mut Registry,
    ) -> Result<Option<VersionedIndex>, Box<dyn std::error::Error>> {
        let view = glm::look_at(
            &glm::vec3(0.0, 0.0, 0.0), 
            &(glm::vec3(0.0, 0.0, 0.0) + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0)
        );
        let projection = glm::ortho(0.0, WIDTH as f32, 0.0, HEIGHT as f32, 0.0, 0.2);

        let (Some(b_box), Some(c_name), mut this_schema) = (
            registry.entities.get::<CBoundingBox2D>(&self.camera),
            registry.entities.get::<CName>(&self.camera),
            SchemaEntity2D::default()
         ) else {
            return Ok(None);
        };

        let vel = (
            self.rand.range(-200, 200) as f32,
            self.rand.range(-200, 200) as f32,
        );
        let color = glm::vec4(
            self.rand.random(),
            self.rand.random(),
            self.rand.random(),
            1.0
        );
        let s_factor = self.rand.range(5, 25) as f32 / 100.0;
        let transform = CTransform2D {
            translate: glm::vec2(
                // self.rand.range(0 + 100, b_box.right as i32 - 100) as f32,
                // self.rand.range(0 + 100, b_box.top as i32 - 100) as f32,
                b_box.right / 2.0,
                b_box.top / 2.0
            ),
            scale: glm::vec2(s_factor, s_factor),
            ..CTransform2D::default()
        };
        let quad = CQuad {
            width: 200.0,
            height: 200.0,
            color,
            mvp: projection * view * transform.to_matrix(),
            ..CQuad::default()
        };

        this_schema.velocity = Some(CVelocity2D { x: vel.0, y: vel.1 });
        this_schema.transform = transform;
        this_schema.quad = quad;
        this_schema.tag = CTag { tag: "bubble".into() };
        this_schema.shader = CName { name: "sprite".into() };
        this_schema.camera = c_name.clone();
        this_schema.texture = Some(CName { name: "Sprite-0001.png".into() });
        this_schema.is_static = false;

        let id = this_schema.build(registry)?;

        Ok(Some(id))
    }
}

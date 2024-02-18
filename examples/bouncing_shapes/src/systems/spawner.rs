use quipi::{
    components::{
        CQuad, CTransform2D, CVelocity2D
    },
    schemas::{
        ISchema, SchemaSprite
    },
    Registry,
    VersionedIndex
};
use quipi_core::{
    core::canvas::get_dimensions,
    math::random::Random,
    utils::now_secs
};

pub struct RectSpawner {
    rand: Random
}

impl RectSpawner {
    pub fn new() -> Result<RectSpawner, Box<dyn std::error::Error>> {
        Ok(Self {
            rand: Random::from_seed(now_secs()?)
        })
    }

    pub fn spawn(
        &mut self,
        registry: &mut Registry,
    ) -> Result<Option<VersionedIndex>, Box<dyn std::error::Error>> {
        let mut this_schema = SchemaSprite::default();

        let (_x, _y, width, height) = get_dimensions();
        let view = glm::look_at(
            &glm::vec3(0.0, 0.0, 0.0), 
            &(glm::vec3(0.0, 0.0, 0.0) + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0)
        );
        let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, 0.0, 0.2);

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
                width as f32 / 2.0,
                height as f32 / 2.0
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
        this_schema.tag = "bubble".into();
        this_schema.shader = "sprite".into();
        this_schema.texture = Some("Sprite-0001.png".into());

        let id = this_schema.build_entity(registry)?;

        Ok(Some(id))
    }
}

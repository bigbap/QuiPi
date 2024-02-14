use quipi::{
    components::{
        CBoundingBox2D, CRect, CTransform2D, CVelocity2D, CRGBA
    },
    schemas::{
        ISchema, SchemaEntity2D
    },
    Registry,
    VersionedIndex
};
use quipi_core::{components::{CElementArray, CName, CTag}, core::rendering::mesh::{ElementArray, ShaderLocation}, math::random::Random, opengl::buffer::BufferUsage, utils::now_secs};

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
        let color = [
            self.rand.random(),
            self.rand.random(),
            self.rand.random(),
            1.0
        ];
        let rect = CRect {
            width: 256.0,
            height: 256.0,
            ..CRect::default()
        };
        let s_factor = self.rand.range(10, 50) as f32 / 100.0;
        let transform = CTransform2D {
            translate: glm::vec2(
                self.rand.range(0, b_box.right as i32) as f32,
                self.rand.range(0, b_box.top as i32) as f32,
                // b_box.right / 2.0,
                // b_box.top / 2.0
            ),
            scale: glm::vec2(s_factor, s_factor),
            ..CTransform2D::default()
        };

        // this_schema.velocity = Some(CVelocity2D { x: vel.0, y: vel.1 });
        this_schema.color = Some(CRGBA { value: color });
        this_schema.transform = transform;
        this_schema.rect = rect;
        this_schema.tag = CTag { tag: "bubble".into() };
        this_schema.shader = CName { name: "sprite".into() };
        this_schema.camera = c_name.clone();
        this_schema.texture = Some(CName { name: "Sprite-0001.png".into() });
        this_schema.is_static = false;

        let id = this_schema.build(registry)?;

        let mesh_data = this_schema.rect.to_mesh(this_schema.color);
        let mut element_arr = ElementArray::new(mesh_data.indices.len(), BufferUsage::StaticDraw)?;
        element_arr
            .with_ebo(&mesh_data.indices)?
            .with_vbo::<3, f32>(ShaderLocation::Zero, &mesh_data.vertices)?
            .with_vbo::<4, f32>(ShaderLocation::One, &mesh_data.colors)?
            .with_vbo::<2, f32>(ShaderLocation::Two, &mesh_data.tex_coords)?;

        registry.entities.add(&id, CElementArray(element_arr));

        Ok(Some(id))
    }
}

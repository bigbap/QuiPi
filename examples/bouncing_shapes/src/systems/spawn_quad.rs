use skald::{
    Registry,
    VersionedIndex,
    gfx::ElementArrayMesh,
    components::{
        CModelNode,
        CVelocity,
        CTransform,
        CModelMatrix,
        CQuadConfig,
        CBoundingBox
    },
    math::random::Random
};

use crate::{WIDTH, HEIGHT};

pub fn s_spawn_quad(
    registry: &mut Registry,
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let config = CQuadConfig {
        width: 256.0,
        height: 256.0,
        center_x: 0.0,
        center_y: 0.0
    };

    s_create_quad(
        registry,
        config,
        (rand.random(), rand.random(), rand.random(), 0.5),
        rand
    )
}

pub fn s_create_quad(
    registry: &mut Registry,
    config: CQuadConfig,
    color: (f32, f32, f32, f32),
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let obj_config = config.to_obj_config(color);
    let mesh = ElementArrayMesh::new(&obj_config.indices)?;
    mesh
        .create_vbo_at(&obj_config.points, 0, 3)?
        .create_vbo_at(&obj_config.colors, 1, 4)?;

    let mut vel = (
        rand.range(0, 200) as f32,
        rand.range(0, 200) as f32,
    );
    if rand.random() > 0.5 { vel.0 *= -1.0; }
    if rand.random() > 0.5 { vel.1 *= -1.0; }

    println!("{:?}", vel);
    
    let quad = registry.create_entity("quad")?
        .with(CModelNode {
            mesh: Some(mesh),
            ..CModelNode::default()
        })?
        .with(CBoundingBox {
            right: config.width,
            bottom: config.height,
            ..CBoundingBox::default()
        })?
        .with(CVelocity {
            x: vel.0,
            y: vel.1,
            z: 0.0
        })?
        .with(CTransform {
            translate: glm::vec3(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5, 0.0),
            scale: Some(glm::vec3(0.2, 0.2, 0.0)),
            ..CTransform::default()
        })?
        .with(CModelMatrix::default())?
        .done()?;

    println!("{}", registry.entity_count());

    Ok(quad)
}

pub fn s_create_circle(
    _registry: &mut Registry,
    parts: &[f32],
) {
    let [radius, center_x, center_y, r, g, b] = parts else { todo!() };

    println!("{radius}, {center_x}, {center_y}, {r}, {g}, {b}");
}

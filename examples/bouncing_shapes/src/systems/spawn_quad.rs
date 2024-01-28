use skald::{
    Registry,
    VersionedIndex,
    gfx::{
        ElementArrayMesh,
        mesh::{BufferUsage, ShaderLocation}
    },
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
        width: 128.0,
        height: 128.0,
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
    let mut mesh = ElementArrayMesh::new(
        obj_config.indices.len(),
        BufferUsage::StaticDraw
    )?;
    mesh
        .with_ebo(&obj_config.indices)?
        .with_vbo::<3, f32>(ShaderLocation::Zero, &obj_config.points)?
        .with_vbo::<4, f32>(ShaderLocation::One, &obj_config.colors)?;

    let mut vel = (
        rand.range(0, 200) as f32,
        rand.range(0, 200) as f32,
    );
    if rand.random() > 0.5 { vel.0 *= -1.0; }
    if rand.random() > 0.5 { vel.1 *= -1.0; }
    
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

    Ok(quad)
}

pub fn s_create_circle(
    _registry: &mut Registry,
    parts: &[f32],
) {
    let [radius, center_x, center_y, r, g, b] = parts else { todo!() };

    println!("{radius}, {center_x}, {center_y}, {r}, {g}, {b}");
}

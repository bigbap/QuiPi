use engine::{
    Registry,
    VersionedIndex,
    gfx::{
        ElementArrayMesh,
        utils::normalise_dims_2d
    },
    components::{
        CModelNode,
        CVelocity,
        CTransform,
        CDimensions,
        CModelMatrix
    },
    math::random::Random
};

use crate::{WIDTH, HEIGHT};

pub fn s_spawn_quad(
    registry: &mut Registry,
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let width = rand.range(200, 400) as f32;
    let height = rand.range(200, 300) as f32;
    s_create_quad(
        registry,
        &[width, height, 0.0, 0.0, rand.random(), rand.random(), rand.random()],
        rand
    )
}

pub fn s_create_quad(
    registry: &mut Registry,
    parts: &[f32],
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let [width, height, center_x, center_y, r, g, b] = parts else { todo!() };
    let (width, height) = normalise_dims_2d(*width, *height, WIDTH as f32, HEIGHT as f32);
    let (pos_x, pos_y) = normalise_dims_2d(*center_x, *center_y, WIDTH as f32, HEIGHT as f32);

    let points: Vec<f32> = vec![
        pos_x, pos_y, 0.0,
        pos_x + width, pos_y, 0.0,
        pos_x + width, pos_y + height, 0.0,
        pos_x, pos_y + height, 0.0
    ];

    let r = *r;
    let g = *g;
    let b = *b;
    let color: Vec<f32> = vec![
        r, g, b,
        r, g, b,
        r, g, b,
        r, g, b
    ];
    let indices = vec![
        0, 1, 2,
        3, 0, 2
    ];

    let mesh = ElementArrayMesh::new(&indices)?;
    mesh
        .create_vbo_at(&points, 0, 3)?
        .create_vbo_at(&color, 1, 3)?;

    let vel = (
        ((rand.random() * 2.0) - 1.0) * 1.0,
        ((rand.random() * 2.0) - 1.0) * 1.0,
    );
    let quad = registry.create_entity("quad")?
        .with(CModelNode {
            mesh: Some(mesh),
            ..CModelNode::default()
        })?
        .with(CDimensions {
            width,
            height,
            ..CDimensions::default()
        })?
        .with(CVelocity {
            x: vel.0,
            y: vel.1,
            z: 0.0
        })?
        .with(CTransform {
            translate: Some(glm::vec3(0.0, 0.0, 0.0)),
            scale: Some(glm::vec3(0.5, 0.5, 0.5)),
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
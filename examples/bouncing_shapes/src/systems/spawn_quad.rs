use engine::{
    Registry,
    VersionedIndex,
    gfx::{
        ElementArrayMesh,
        utils::normalise_dims
    },
    components::{
        CMesh,
        CPosition,
        CVelocity,
        CTransform
    },
    math::random::Random
};

pub fn s_spawn_quad(
    registry: &mut Registry,
    clr: (f32, f32, f32),
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    // let mut color = vec![];
    // for _ in 0..6 {
    //     color.push(rand.random());
    //     color.push(rand.random());
    //     color.push(rand.random());
    // }
    // let indices = vec![
    //     0, 1, 2,
    //     3, 0, 2
    // ];
    // let points = vec![
    //     -0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0,
    //     -0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, -0.5, 0.0
    // ];
    //
    // let mesh = ElementArrayMesh::new(&indices)?;
    // mesh
    //     .create_vbo_at(&points, 0, 3)?
    //     .create_vbo_at(&color, 1, 3)?;
    //
    // let quad = registry.create_entity("quad")?
    //     .with(CMesh { mesh })?
    //     .with(CPosition { x: 0.0, y: 0.0, z: 0.0 })?
    //     .with(CVelocity { x: 0.02, y: 0.02, z: 0.0 })?
    //     .with(CTransform {
    //         translate: Some(glm::vec3(0.0, 0.0, 0.0)),
    //         scale: Some(glm::vec3(0.5, 0.5, 0.5)),
    //         ..CTransform::default()
    //     })?
    //     .done()?;
    //
    // Ok(quad)

    let width = rand.random() * 1000.0;
    let height = rand.random() * 1000.0;
    let center_x = rand.range(0, 800) as f32;
    let center_y = rand.range(0, 600) as f32;
    s_create_quad(
        registry,
        &[width, height, center_x, center_y, rand.random(), rand.random(), rand.random()],
        800.0,
        600.0,
        rand
    )
}

pub fn s_create_quad(
    registry: &mut Registry,
    parts: &[f32],
    screen_width: f32,
    screen_height: f32,
    rand: &mut Random
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let [width, height, center_x, center_y, r, g, b] = parts else { todo!() };
    let (width, height) = normalise_dims(*width, *height, screen_width, screen_height);
    let (center_x, center_y) = normalise_dims(*center_x, *center_y, screen_width, screen_height);

    let top_left = (center_x - (width / 2.0), center_y + (height / 2.0));
    let bottom_left = (center_x - (width / 2.0), center_y - (height / 2.0));
    let top_right = (center_x + (width / 2.0), center_y + (height / 2.0));
    let bottom_right = (center_x + (width / 2.0), center_y - (height / 2.0));

    let points: Vec<f32> = vec![
        top_left.0, top_left.1, 0.0, top_right.0, top_right.1, 0.0, bottom_right.0, bottom_right.1, 0.0,
        bottom_left.0, bottom_left.1, 0.0, top_left.0, top_left.1, 0.0, bottom_right.0, bottom_right.1, 0.0
    ];

    let r = *r;
    let g = *g;
    let b = *b;
    let color: Vec<f32> = vec![
        r, g, b, r, g, b, r, g, b,
        r, g, b, r, g, b, r, g, b
    ];
    let indices = vec![
        0, 1, 2,
        3, 0, 2
    ];

    let mesh = ElementArrayMesh::new(&indices)?;
    mesh
        .create_vbo_at(&points, 0, 3)?
        .create_vbo_at(&color, 1, 3)?;

    let quad = registry.create_entity("quad")?
        .with(CMesh { mesh })?
        .with(CPosition { x: 0.0, y: 0.0, z: 0.0 })?
        .with(CVelocity {
            x: ((rand.random() * 2.0) - 1.0) / 10.0,
            y: ((rand.random() * 2.0) - 1.0) / 10.0,
            z: 0.0
        })?
        .with(CTransform {
            translate: Some(glm::vec3(0.0, 0.0, 0.0)),
            scale: Some(glm::vec3(0.5, 0.5, 0.5)),
            ..CTransform::default()
        })?
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

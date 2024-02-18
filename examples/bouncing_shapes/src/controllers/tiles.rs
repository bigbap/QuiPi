use quipi_2d::{components::{CModelMatrix2D, CQuad, CTransform2D}, resources::RCamera2D};
use quipi_core::{
    core::canvas::get_dimensions, math::random::Random, opengl::capabilities::{
        gl_blending_func,
        gl_enable,
        GLBlendingFactor,
        GLCapability
    }, rendering::{
        batch::BatchRenderer,
        RenderInfo
    }, resources::RShader, utils::now_secs, FrameState, IController, Registry, VersionedIndex
};

const CAMERA: &str = "main_camera";
const SHADER: &str = "sprite";

pub struct TileControler {
    tiles: Vec<VersionedIndex>,

    camera: u64,
    shader: u64,

    renderer: BatchRenderer<1000, CQuad>,
}

impl TileControler {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(camera) = registry.get_resource_id(CAMERA) else {
            return Err("[bubble controller] camera is not loaded".into());
        };

        let Some(shader) = registry.get_resource_id(SHADER) else {
            return Err("[bubble controller] shader is not loaded".into());
        };

        let (_x, _y, width, height) = get_dimensions();
        let mut rand = Random::from_seed(now_secs()?);
        let mut tiles = vec![];
        for x in 0..(width / 10) {
            for y in 0..(height / 10) {
                tiles.push(tile(x as u32, y as u32, &mut rand, registry));
            }
        }

        Ok(Self {
            tiles,
            camera,
            shader,
            renderer: BatchRenderer::new(),
        })
    }
}

impl IController for TileControler {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> Option<RenderInfo> {
        if registry.get_resource::<RShader>(self.shader).is_none() {
            #[cfg(debug_assertions)]
            println!("[tile controller] tried to use a shader that is not loaded");

            return None;
        };

        let Some(camera) = registry.get_resource::<RCamera2D>(self.camera) else {
            #[cfg(debug_assertions)]
            println!("[tile controller] tried to use a camera that is not loaded");

            return None;
        };

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        let view = RCamera2D::calc_view_matrix(&camera.transform);
        let projection = RCamera2D::calc_projection_matrix(&camera.params);

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for tile in self.tiles.iter() {
            let Some(model) = registry.entities.get::<CModelMatrix2D>(&tile) else {
                #[cfg(debug_assertions)]
                println!("[tile controller] tried to render a tile without a model matrix");

                continue;
            };

            let mvp = projection * view * model.0;

            let Some(quad) = registry.entities.get_mut::<CQuad>(&tile) else { continue; };
            quad.mvp = mvp;

            let quad = registry.entities.get::<CQuad>(&tile)?;

            self.renderer.draw_mesh(quad, registry.get_resource(self.shader)?, None);
        }
        self.renderer.end_batch();
        self.renderer.flush_batch(registry.get_resource(self.shader)?);

        Some(self.renderer.render_info.clone())
    }
}

fn tile(x: u32, y: u32, rand: &mut Random, registry: &mut Registry) -> VersionedIndex {
    let x_offset = (x + 1) as f32 * 1.0;
    let y_offset = (y + 1) as f32 * 1.0;
    let color = match rand.random() > 0.9 {
        true => glm::vec4(0.3, 0.3, 0.3, 1.0),
        false => glm::vec4(0.6, 0.8, 0.0, 1.0),
    };

    let transform = CTransform2D {
        translate: glm::vec2(15.0 + x_offset, 15.0 + y_offset),
        scale: glm::vec2(50.0, 50.0),
        ..CTransform2D::default()
    };

    let entity = registry.entities.create();
    registry.entities.add(&entity, CQuad {
        center_x: x as f32,
        center_y: y as f32,
        width: 1.0,
        height: 1.0,
        color,
        ..CQuad::default()
    });
    registry.entities.add(&entity, CModelMatrix2D(transform.to_matrix()));

    entity
}
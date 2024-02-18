use quipi_2d::{
    components::{
        CBoundingBox2D, CQuad, CTransform2D, CVelocity2D
    },
    resources::RCamera2D, schemas::{
        ISchema,
        SchemaSprite
    },
    IController
};
use quipi_core::{
    components::{
        CDrawable,
        CTag
    }, core::canvas::get_dimensions, math::random::Random, opengl::capabilities::{
        gl_blending_func,
        gl_enable,
        GLBlendingFactor,
        GLCapability
    }, rendering::{
        batch::BatchRenderer,
        RenderInfo
    }, resources::RShader, utils::now_secs, FrameResponse, FrameState, Registry, VersionedIndex
};
use sdl2::{event::Event, keyboard::Keycode};

const TAG: &str = "bubble";
const CAMERA: &str = "main_camera";
const SHADER: &str = "sprite";

pub struct BubbleController {
    bubbles: Vec<VersionedIndex>,

    camera: u64,
    shader: u64,

    renderer: BatchRenderer<1000, CQuad>,
    rand: Random
}

impl BubbleController {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let bubbles = registry.entities.query(CTag { tag: TAG.to_string() });

        let Some(camera) = registry.get_resource_id(CAMERA) else {
            return Err("[bubble controller] camera is not loaded".into());
        };

        let Some(shader) = registry.get_resource_id(SHADER) else {
            return Err("[bubble controller] shader is not loaded".into());
        };

        let rand = Random::from_seed(now_secs()?);

        // for stress testing
        // for _ in 0..30000 {
        //     bubbles.push(spawn(&mut rand, registry)?);
        // }

        Ok(Self {
            bubbles,
            camera,
            shader,
            renderer: BatchRenderer::new(),
            rand
        })
    }
}

impl IController for BubbleController {
    fn update(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> FrameResponse {
        if frame_state.editor_mode { return FrameResponse::None; }

        // handle input
        for event in frame_state.events.iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    match spawn(&mut self.rand, registry) {
                        Ok(index) => self.bubbles.push(index),
                        Err(_e) => {
                            #[cfg(debug_assertions)]
                            println!("[bubble controller] there was a problem spawning a bubble: {}", _e.to_string());
                        }
                    }

                    break;
                },
                _ => ()
            };
        }

        update(&self.bubbles, registry, frame_state);

        FrameResponse::None
    }

    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> Option<RenderInfo> {
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        if registry.get_resource::<RShader>(self.shader).is_none() {
            #[cfg(debug_assertions)]
            println!("[bubble controller] tried to use a shader that is not loaded");

            return None;
        };

        let Some(camera) = registry.get_resource::<RCamera2D>(self.camera) else {
            #[cfg(debug_assertions)]
            println!("[bubble controller] tried to use a camera that is not loaded");

            return None;
        };

        let view = RCamera2D::calc_view_matrix(&camera.transform);
        let projection = RCamera2D::calc_projection_matrix(&camera.params);

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for bubble in self.bubbles.iter() {
            let Some(transform) = registry.entities.get::<CTransform2D>(&bubble) else {
                #[cfg(debug_assertions)]
                println!("[bubble controller] tried to render a bubble without a transform");

                continue;
            };

            let model = transform.to_matrix();
            let mvp = projection * view * model;

            let Some(quad) = registry.entities.get_mut::<CQuad>(&bubble) else { continue; };
            quad.mvp = mvp;

            let quad = registry.entities.get::<CQuad>(&bubble).unwrap();
            let drawable = registry.entities.get::<CDrawable>(&bubble).unwrap();
            let texture = match drawable.texture {
                Some(id) => registry.get_resource(id),
                _ => None
            };

            self.renderer.draw_mesh(quad, registry.get_resource(self.shader).unwrap(), texture);
        }
        self.renderer.end_batch();
        self.renderer.flush_batch(registry.get_resource(self.shader).unwrap());

        Some(self.renderer.render_info.clone())
    }
}

fn spawn(
    rand: &mut Random,
    registry: &mut Registry,
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let mut this_schema = SchemaSprite::default();

    let (_x, _y, width, height) = get_dimensions();

    let vel = (
        rand.range(-200, 200) as f32,
        rand.range(-200, 200) as f32,
    );
    let color = glm::vec4(
        rand.random(),
        rand.random(),
        rand.random(),
        1.0
    );
    let s_factor = rand.range(5, 25) as f32 / 100.0;
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
        ..CQuad::default()
    };

    this_schema.velocity = Some(CVelocity2D { x: vel.0, y: vel.1 });
    this_schema.transform = transform;
    this_schema.quad = quad;
    this_schema.tag = TAG.into();
    this_schema.shader = SHADER.into();
    this_schema.texture = Some("Sprite-0001.png".into());
    this_schema.camera = CAMERA.to_string();

    let id = this_schema.build_entity(registry)?;

    Ok(id)
}

pub fn update(
    bubbles: &[VersionedIndex],
    registry: &mut Registry,
    frame_state: &mut FrameState
) {
    for bubble in bubbles {
        let Some(vel)       = registry.entities.get::<CVelocity2D>(&bubble)    else { continue };
        let Some(transform) = registry.entities.get::<CTransform2D>(&bubble)   else { continue };
        let Some(b_box)     = registry.entities.get::<CBoundingBox2D>(&bubble) else { continue };
        
        let scale = transform.scale;

        let vel = glm::vec2(vel.x, vel.y);
        let translate = transform.translate + (vel * frame_state.delta);
        let w = b_box.right * scale.x;
        let h = b_box.bottom * scale.y;
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            w,
            h
        );

        let (_x, _y, width, height) = get_dimensions();
        let Some(transform) = registry.entities.get_mut::<CTransform2D>(&bubble) else { continue };
        match colided_x {
            -1 => transform.translate.x = 0.0 + w * 0.5,
            1 => transform.translate.x = width as f32 - w * 0.5,
            _ => transform.translate.x = translate.x
        }
        match colided_y {
            -1 => transform.translate.y = 0.0 + h * 0.5,
            1 => transform.translate.y = height as f32 - h * 0.5,
            _ => transform.translate.y = translate.y
        }

        let Some(vel) = registry.entities.get_mut::<CVelocity2D>(&bubble) else { continue };
        if colided_x != 0 { vel.x *= -1.0 }
        if colided_y != 0 { vel.y *= -1.0 }
    }
}

fn check_screen_collision(
    pos: glm::Vec2,
    w: f32,
    h: f32
) -> (i32, i32) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let (_x, _y, width, height) = get_dimensions();

    let mut colided_x = 0;
    let mut colided_y = 0;

    if pos.x <= (0.0 + offset_x) { colided_x = -1; }
    if pos.x >= (width as f32 - offset_x) { colided_x = 1; }
    if pos.y >= (height as f32 - offset_y) { colided_y = 1; }
    if pos.y <= (0.0 + offset_y) {  colided_y = -1; }

    (colided_x, colided_y)
}
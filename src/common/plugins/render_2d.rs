use crate::{
    common::{
        assets::{ShaderAsset, ShaderLoader, TextureAsset, TextureCoords},
        bundles::CSprite,
        prelude::components::{CQuad, CSkip, CTextureAtlas, CTransform2D},
        resources::{Camera2D, CameraId, Source},
    },
    platform::opengl::capabilities::{gl_blending_func, gl_enable, GLBlendingFactor, GLCapability},
    prelude::{
        qp_common::resources::AssetId,
        qp_gfx::{BatchRenderer, Vertex},
        QPError, System,
    },
    registry::GlobalRegistry,
    world::RenderSchedule,
};

use super::Plugin;

const BATCH_SIZE: usize = 10000;
const SHADER_NAME: &str = "quad_shader";
const CAMERA_NAME: &str = "default_quad_camera";

pub struct Render2DPlugin {
    camera: Camera2D,
    camera_name: &'static str,
}

impl Default for Render2DPlugin {
    fn default() -> Self {
        Self {
            camera: Camera2D::default(),
            camera_name: CAMERA_NAME,
        }
    }
}

impl Plugin for Render2DPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        let shader_id = app.world.registry.resources.load_asset(
            SHADER_NAME,
            ShaderLoader {
                source: Source::Strings((VERT, FRAG)),
                uniforms: vec![],
            },
        )?;

        let camera_id = app
            .world
            .registry
            .resources
            .add_camera(self.camera_name, self.camera.clone())?;

        app.add_system::<RenderSchedule>(quad_render_system(shader_id, camera_id));

        Ok(())
    }
}

pub fn quad_render_system(shader_id: AssetId, camera_id: CameraId) -> impl System {
    debug_assert!(shader_id.validate::<ShaderAsset>());

    let mut renderer = BatchRenderer::<BATCH_SIZE, 4>::new(vec![0, 1, 3, 1, 2, 3]);

    move |registry: &mut GlobalRegistry| {
        let Some(entities) = registry.entities.query::<CSprite>() else {
            return Ok(());
        };

        let shader = registry
            .resources
            .get_asset(&shader_id)
            .ok_or(QPError::AssetNotFound(SHADER_NAME.into()))?;

        let camera = registry
            .resources
            .get_camera::<Camera2D>(&camera_id)
            .ok_or(QPError::CameraNotFound)?;

        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(
            GLBlendingFactor::SrcAlpha,
            GLBlendingFactor::OneMinusSrcAlpha,
        );

        renderer.reset_info();
        renderer.begin_batch();
        for (entity, sprite) in entities.iter() {
            if sprite.is_none()
                || registry.entities.get::<CSkip>(&entity).is_some()
                || registry.entities.get::<CTransform2D>(&entity).is_none()
                || registry.entities.get::<CQuad>(&entity).is_none()
            {
                continue;
            }

            let quad = registry.entities.get::<CQuad>(&entity).unwrap();
            let transform = registry.entities.get::<CTransform2D>(&entity).unwrap();
            let model = transform.to_matrix();

            let mvp = camera.projection.0 * camera.view.0 * model;

            let texture = match registry.entities.get::<CTextureAtlas>(&entity) {
                Some(atlas) => registry.resources.get_asset(&atlas.texture),
                _ => None,
            };

            renderer.draw(
                vertices(
                    &mvp,
                    &texture,
                    glm::vec4(1.0, 1.0, 1.0, 1.0),
                    quad.positions(),
                ),
                shader,
                texture,
            );
        }
        renderer.end_batch();
        renderer.flush_batch(shader);

        Ok(())
    }
}

pub fn vertices(
    mvp: &glm::Mat4,
    texture: &Option<&TextureAsset>,
    color: glm::Vec4,
    positions: [glm::Vec4; 4],
) -> [Vertex; 4] {
    let pos1 = mvp * positions[0];
    let pos2 = mvp * positions[1];
    let pos3 = mvp * positions[2];
    let pos4 = mvp * positions[3];

    let coords = match texture {
        Some(tex) => tex.get_coords_at_loc((1, 1)),
        _ => TextureCoords::default(),
    };

    [
        Vertex {
            position: pos1.xyz(),
            color,
            tex_coords: coords.top_right,
            tex_index: 0.0,
        },
        Vertex {
            position: pos2.xyz(),
            color,
            tex_coords: coords.bottom_right,
            tex_index: 0.0,
        },
        Vertex {
            position: pos3.xyz(),
            color,
            tex_coords: coords.bottom_left,
            tex_index: 0.0,
        },
        Vertex {
            position: pos4.xyz(),
            color,
            tex_coords: coords.top_left,
            tex_index: 0.0,
        },
    ]
}

const VERT: &str = r#"
#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in float aTexIndex;

out vec4 color;
out vec2 texCoords;
out float texIndex;

void main(){
    gl_Position = vec4(aPos, 1.0);

    color = aColor;
    texCoords = aTexCoords;
    texIndex = aTexIndex;
}
"#;

const FRAG: &str = r#"
#version 450 core

in vec4 color;
in vec2 texCoords;
in float texIndex;

uniform sampler2D u_textures[32];

out vec4 fragColor;

void main() {
    int texId = int(texIndex);

    if (texId >= 32) {
        fragColor = color;
    } else {
        fragColor = color * texture(u_textures[texId], texCoords);
    }
}
"#;

use crate::{
    assets::{AssetServer, Assets, Source},
    common::resources::StringInterner,
    gfx::render::assets::{Shader, ShaderLoader},
    plugin::Plugin,
    prelude::ResMut,
    schedule::Startup,
};

pub const QUAD_SHADER_NAME: &str = "default_quad_shader";

#[derive(Default)]
pub struct DefaultQuadShader;

impl Plugin for DefaultQuadShader {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        app.add_system(
            Startup,
            |asset_server: ResMut<AssetServer>,
             store: ResMut<Assets<Shader>>,
             interner: ResMut<StringInterner>| {
                if let (Some(server), Some(store), Some(interner)) = (asset_server, store, interner)
                {
                    store.add(
                        interner.intern(QUAD_SHADER_NAME),
                        server
                            .load(ShaderLoader {
                                source: Source::Strings((VERT, FRAG)),
                                uniforms: vec![],
                            })
                            .unwrap(),
                    );
                }
            },
        );

        Ok(())
    }
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

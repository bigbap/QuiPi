use crate::{
    assets::{AssetServer, Assets, Source},
    core::prelude::StringInterner,
    gfx::render::assets::{Shader, ShaderLoader},
    plugin::Plugin,
    prelude::ResMut,
    schedule::Startup,
};

pub const TEXT_SHADER_NAME: &str = "text_shader";

#[derive(Default)]
pub struct TextShader;

impl Plugin for TextShader {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        app.add_system(
            Startup,
            |asset_server: ResMut<AssetServer>,
             store: ResMut<Assets<Shader>>,
             interner: ResMut<StringInterner>| {
                if let (Some(server), Some(store), Some(interner)) = (asset_server, store, interner)
                {
                    store.add(
                        interner.intern(TEXT_SHADER_NAME),
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

void main()
{
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

void main()
{
    int texId = int(texIndex);
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(u_textures[texId], texCoords).r);
    fragColor = color * sampled;
}
"#;

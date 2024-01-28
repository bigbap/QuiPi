use super::buffer::BUFFER_FLAGS;

// https://registry.khronos.org/OpenGL-Refpages/gl4/html/glEnable.xhtml
#[derive(Debug, PartialEq, Eq)]
pub enum GLCapability {
    DepthTest,
    AlphaBlending,
    ScissorTest,
    StencilTest,
    FrameBufferSRGB
}

pub fn enable(flag: GLCapability) {
    unsafe {
        gl::Enable(match flag {
            GLCapability::DepthTest             => gl::DEPTH_TEST,
            GLCapability::AlphaBlending         => gl::BLEND,
            GLCapability::ScissorTest           => gl::SCISSOR_TEST,
            GLCapability::StencilTest           => gl::STENCIL_TEST,
            GLCapability::FrameBufferSRGB       => gl::FRAMEBUFFER_SRGB,
        });

        if flag == GLCapability::DepthTest {
            BUFFER_FLAGS |= gl::DEPTH_BUFFER_BIT;
        }
    }
}

pub fn disable(flag: GLCapability) {
    unsafe {
        gl::Disable(match flag {
            GLCapability::DepthTest             => gl::DEPTH_TEST,
            GLCapability::AlphaBlending         => gl::BLEND,
            GLCapability::ScissorTest           => gl::SCISSOR_TEST,
            GLCapability::StencilTest           => gl::STENCIL_TEST,
            GLCapability::FrameBufferSRGB       => gl::FRAMEBUFFER_SRGB,
        });

        if flag == GLCapability::DepthTest {
            BUFFER_FLAGS &= gl::DEPTH_BUFFER_BIT;
        }
    }
}

// blending
pub enum BlendingFactor {
    SrcAlpha,
    OneMinusSrcAlpha,
    One,
}

pub fn blending_func(
    s_factor: BlendingFactor,
    d_factor: BlendingFactor
) {
    unsafe { gl::BlendFunc(s_factor.unwrap(), d_factor.unwrap()) }
}

impl BlendingFactor {
    fn unwrap(&self) -> gl::types::GLenum {
        match self {
            BlendingFactor::One                 => gl::ONE,
            BlendingFactor::SrcAlpha            => gl::SRC_ALPHA,
            BlendingFactor::OneMinusSrcAlpha    => gl::ONE_MINUS_SRC_ALPHA
        }
    }
}

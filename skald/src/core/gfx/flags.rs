pub enum GFXFlags {
    DepthTest,
    DepthMask,
    AlphaBlending,
}

pub fn set_flags(flags: &[GFXFlags]) {
    for flag in flags.iter() {
        match flag {
            GFXFlags::DepthTest => unsafe { gl::Enable(gl::DEPTH_TEST) },
            GFXFlags::DepthMask => unsafe { gl::DepthMask(gl::TRUE) },
            GFXFlags::AlphaBlending => unsafe {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
            },
        }
    }
}

pub fn unset_flags(flags: &[GFXFlags]) {
    for flag in flags.iter() {
        match flag {
            GFXFlags::DepthTest => unsafe { gl::Disable(gl::DEPTH_TEST) },
            GFXFlags::DepthMask => unsafe { gl::DepthMask(gl::FALSE) },
            GFXFlags::AlphaBlending => unsafe { gl::Disable(gl::BLEND) },
        }
    }
}

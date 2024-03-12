use crate::resources::ResourceManager;
use std::ptr;

pub struct World {
    pub resources: ResourceManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: ResourceManager::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: u32,
    pub frame_ms: u32,

    pub editor_ms: u32,
    pub controller_ms: u32,
    pub render_ms: u32,
    pub draw_calls: u32,
}

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell(pub *mut World);

impl UnsafeWorldCell {
    #[inline]
    pub(crate) fn new_readonly(world: &'static World) -> Self {
        Self(std::ptr::from_ref(world).cast_mut())
    }

    #[inline]
    pub(crate) fn new_mutable(world: &'static mut World) -> Self {
        Self(ptr::from_mut(world))
    }
}

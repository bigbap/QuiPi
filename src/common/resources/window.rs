use crate::{
    platform::sdl2::QPWindow,
    prelude::{
        qp_gfx::{self, Viewport},
        QPError,
    },
    resources::*,
    QPResult,
};

#[derive(Resource)]
pub struct Window {
    pub winapi: QPWindow,
    pub viewport: Viewport,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> QPResult<Self> {
        let mut winapi = QPWindow::init().unwrap();
        let _window = winapi.opengl_window(title, width, height, (4, 5))?;

        qp_gfx::init(&winapi).map_err(|e| QPError::Generic(e.to_string()))?;

        let viewport = Viewport::new(0, 0, width as i32, height as i32);

        Ok(Self { winapi, viewport })
    }
}

use quipi::engine::QuiPiApp;

extern crate quipi;
extern crate nalgebra_glm as glm;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub type SandboxError = Box<dyn std::error::Error>;

pub struct QuiPiSandbox {}

impl QuiPiSandbox {
    pub fn new() -> Result<Self, SandboxError> {
        Ok(Self {})
    }
}

impl QuiPiApp for QuiPiSandbox {
    fn init(
        &mut self,
        _gui: Option<quipi::wrappers::egui::GUI>
    ) -> Result<(), SandboxError> {
        Ok(())
    }

    fn handle_frame(
        &mut self,
        _event_pump: &mut quipi::sdl2::EventPump
    ) -> Result<Option<()>, SandboxError> {
        Ok(Some(()))
    }
}

mod backend;

#[cfg(feature = "qp_editor")]
pub mod prelude {
    use egui::Context;
    use super::backend::prelude::*;
    use crate::QPResult;
    use crate::prelude::{
        qp_core::Timer,
        qp_data::{FrameResponse, FrameState, IController},
        Registry
    };
    
    pub struct GuiManager {
        backend: EguiBackend,
        timer: Timer,
    
        controllers: Vec<Box<dyn IGuiController>>
    }
    
    impl GuiManager {
        pub fn new(scale: f32) -> QPResult<Self> {
            Ok(Self {
                backend: EguiBackend::new(scale)?,
                timer: Timer::new(),
                controllers: vec![]
            })
        }

        pub fn register_controller(&mut self, controller: impl IGuiController + 'static) {
            self.controllers.push(Box::new(controller));
        }

        pub fn ctx(&self) -> &Context {
            &self.backend.ctx
        }
    }
    
    impl IController for GuiManager {
        fn update(
            &mut self,
            frame_state: &mut FrameState,
            registry: &mut Registry,
        ) -> FrameResponse {
            self.timer.delta();
    
            self.backend.begin_frame();
            
            for controller in self.controllers.iter_mut() {
                controller.update(&self.backend.ctx, frame_state, registry);
            }
    
            self.backend.end_frame(frame_state);
    
            frame_state.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;
    
            FrameResponse::None
        }
    }

    pub trait IGuiController {
        fn update(
            &mut self,
            ctx: &Context,
            frame_state: &mut FrameState,
            registry: &mut Registry
        );
    }
}

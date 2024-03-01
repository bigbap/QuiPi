#[cfg(feature = "qp_editor")]
mod input;
#[cfg(feature = "qp_editor")]
mod painter;

#[cfg(feature = "qp_editor")]
pub mod prelude {
    use super::{input::parse_event, painter::Painter};
    use crate::registry::GlobalRegistry;
    use crate::QPResult;
    use egui::RawInput;
    use sdl2::event::{Event, WindowEvent};

    pub struct EguiBackend {
        pub ctx: egui::Context,
        painter: Painter,
        raw_input: RawInput,
    }

    impl EguiBackend {
        pub fn new(scale: f32) -> QPResult<Self> {
            let ctx = egui::Context::default();
            let painter = Painter::new(scale)?;
            let raw_input = RawInput::default();

            Ok(Self {
                ctx,
                painter,
                raw_input,
            })
        }

        pub fn begin_frame(&mut self) {
            self.ctx.begin_frame(self.raw_input.take());
        }

        pub fn end_frame(&mut self, registry: &mut GlobalRegistry) {
            let full_output = self.ctx.end_frame();

            self.painter.paint(&self.ctx, full_output);

            self.handle_input(registry)
        }

        fn handle_input(&mut self, registry: &mut GlobalRegistry) {
            for event in registry.events.iter() {
                match event {
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(width, height)
                        | WindowEvent::SizeChanged(width, height) => {
                            registry.gfx.viewport.set_dimensions(0, 0, *width, *height);
                            self.painter.update_screen_rect();
                            self.raw_input.screen_rect = Some(self.painter.screen_rect);
                        }
                        _ => (),
                    },
                    _ => {
                        if let Some(parsed) = parse_event(&event, &self.painter) {
                            self.raw_input.events.push(parsed);
                        }
                    }
                }
            }
        }
    }
}

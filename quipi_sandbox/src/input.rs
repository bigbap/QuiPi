use quipi::{
    FrameState,
    sdl2::{
        event::Event,
        keyboard::{Keycode, Mod}
    },
    schemas::SchemaScene,
    systems::scene::save_scene,
    FrameResponse
};


pub fn handle_input(
    frame_state: &mut FrameState,
    scene: &Option<SchemaScene>
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in frame_state.events.iter() {
        match event {
            Event::Quit { .. } => {

                return Ok(FrameResponse::Quit)
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => frame_state.editor_mode = !frame_state.editor_mode,
            Event::KeyDown { keycode: Some(Keycode::S), keymod: Mod::LCTRLMOD, .. } => {
                if let Some(scene) = scene {
                    save_scene("start", scene)?;
                }
            },
            _ => ()
        }
    }

    Ok(FrameResponse::None)
}

use quipi::{
    AppState,
    sdl2::{
        event::Event,
        keyboard::{Keycode, Mod}
    },
    schemas::SchemaScene,
    systems::scene::save_scene,
    FrameResponse
};


pub fn handle_input(
    app_state: &mut AppState,
    scene: &Option<SchemaScene>
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in app_state.events.iter() {
        match event {
            Event::Quit { .. } => {

                return Ok(FrameResponse::Quit)
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => app_state.editor_mode = !app_state.editor_mode,
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

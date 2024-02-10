use quipi::{
    sdl2::{
        event::Event,
        keyboard::{Keycode, Mod}
    },
    systems::scene::save_scene_2d,
    FrameResponse,
    FrameState,
    Registry,
    VersionedIndex
};


pub fn handle_input(
    frame_state: &mut FrameState,
    scene: VersionedIndex,
    registry: &Registry
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in frame_state.events.iter() {
        match event {
            Event::Quit { .. } => {

                return Ok(FrameResponse::Quit)
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => frame_state.editor_mode = !frame_state.editor_mode,
            Event::KeyDown { keycode: Some(Keycode::S), keymod: Mod::LCTRLMOD, .. } => {
                save_scene_2d("start", scene, &registry)?;
            },
            _ => ()
        }
    }

    Ok(FrameResponse::None)
}

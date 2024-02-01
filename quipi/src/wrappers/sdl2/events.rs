pub struct EventQueue {
    pub event_pump: sdl2::EventPump
}

impl EventQueue {
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        Self {
            event_pump
        }
    }

    // pub fn next(&self) {
    //     self.event_pump.wait_iter()
    // }
}

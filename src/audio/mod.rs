use std::fs::File;
use std::thread;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

use crate::{core::prelude::to_abs_path, QPResult};

pub struct QPAudio {}

impl QPAudio {
    pub fn new() -> QPResult<Self> {
        Ok(Self {})
    }

    pub fn play(&self) {
        thread::spawn(|| {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            // Add a dummy source of the sake of the example.
            let file = BufReader::new(File::open(to_abs_path("assets/audio/jingles_NES00.ogg").unwrap()).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);

            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        });
    }
}
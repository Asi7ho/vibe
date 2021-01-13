use std::{
    fs::File,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};

use crossbeam::queue::SegQueue;
use vibe_core::decoder::Decoder;

use crate::stream::OutputStream;

struct Controls {
    pause: AtomicBool,
    stopped: AtomicBool,
}
pub struct Player {
    queue: Arc<SegQueue<File>>,
    controls: Arc<Controls>,
}

impl Player {
    #[inline]
    pub fn new() -> Result<Self, ()> {
        let queue = Arc::new(SegQueue::new());
        let controls = Arc::new(Controls {
            pause: AtomicBool::new(false),
            stopped: AtomicBool::new(false),
        });

        Ok(Self { queue, controls })
    }

    #[inline]
    pub fn add_to_queue(&mut self, data: File) {
        self.queue.push(data);
    }

    #[inline]
    pub fn send_next_to_stream(&mut self) {
        let next_audio = self.queue.pop().expect("No next audio");

        let decoder = Decoder::new(next_audio).unwrap();
        let stream = OutputStream::new::<f32>(decoder).unwrap();
        stream.play()
    }

    #[inline]
    pub fn play(&self) {
        self.controls.pause.store(false, Ordering::SeqCst);
    }

    #[inline]
    pub fn pause(&self) {
        self.controls.pause.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn stop(&self) {
        self.controls.stopped.store(true, Ordering::SeqCst);
    }

    #[inline]
    pub fn is_paused(&self) -> bool {
        self.controls.pause.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn is_stopped(&self) -> bool {
        self.controls.stopped.load(Ordering::SeqCst)
    }
}

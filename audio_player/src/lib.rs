use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::sync::{mpsc::{self, Sender, Receiver}};
use std::thread;

pub enum PlayerEvent {
    Play(PathBuf),
    Pause,
    Resume,
    Stop,
    Next,
    Quit,
}

pub struct AudioPlayer {
    control_tx: Sender<PlayerEvent>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<PlayerEvent>();
        let handle = thread::spawn(move || player_thread(rx));
        Self { control_tx: tx, handle: Some(handle) }
    }

    pub fn play(&self, path: PathBuf) {
        let _ = self.control_tx.send(PlayerEvent::Play(path));
    }
    pub fn pause(&self) {
        let _ = self.control_tx.send(PlayerEvent::Pause);
    }
    pub fn resume(&self) {
        let _ = self.control_tx.send(PlayerEvent::Resume);
    }
    pub fn stop(&self) {
        let _ = self.control_tx.send(PlayerEvent::Stop);
    }
    pub fn next(&self) {
        let _ = self.control_tx.send(PlayerEvent::Next);
    }
    pub fn quit(&self) {
        let _ = self.control_tx.send(PlayerEvent::Quit);
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        let _ = self.control_tx.send(PlayerEvent::Quit);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn player_thread(rx: Receiver<PlayerEvent>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink: Option<Sink> = None;

    loop {
        if let Ok(event) = rx.recv() {
            match event {
                PlayerEvent::Play(path) => {
                    if let Some(s) = &sink { s.stop(); }
                    let s = Sink::try_new(&stream_handle).unwrap();
                    let file = BufReader::new(File::open(path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    s.append(source);
                    sink = Some(s);
                }
                PlayerEvent::Pause => {
                    if let Some(s) = &sink { s.pause(); }
                }
                PlayerEvent::Resume => {
                    if let Some(s) = &sink { s.play(); }
                }
                PlayerEvent::Stop => {
                    if let Some(s) = &sink { s.stop(); }
                    sink = None;
                }
                PlayerEvent::Next => {
                    if let Some(s) = &sink { s.stop(); }
                    sink = None;
                    // La logique pour passer à la suivante doit être gérée côté appelant
                }
                PlayerEvent::Quit => {
                    if let Some(s) = &sink { s.stop(); }
                    break;
                }
            }
        }
    }
}
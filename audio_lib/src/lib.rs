use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::Source;
use std::thread;
use std::time::{Duration};
use crossterm::{ExecutableCommand, cursor, terminal};
use std::io::{stdout, Write};
use rand::seq::SliceRandom;
use rand::{rng};
use crossterm::event::{self, Event, KeyCode};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
}

pub struct AudioLibrary {
    pub files: Vec<AudioFile>,
}

pub enum PlaybackControl {
    Next,
    Stop,
    Timeout,
}


impl AudioLibrary {
    pub fn load_from_dir(dir: &str) -> Self {
        let files = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let path = entry.into_path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext = ext.to_lowercase();
                    if ["mp3", "flac", "wav", "ogg", "m4a"].contains(&ext.as_str()) {
                        Some(AudioFile { path })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        AudioLibrary { files }
    }

    pub fn list(&self) -> &Vec<AudioFile> {
        &self.files
    }

    pub fn list_strings(&self) -> Vec<String> {
        self.files.iter()
            .map(|file| file.path.file_name().unwrap_or_default().to_string_lossy().to_string())
            .collect()  
    }

    pub fn shuffled(&self) -> Vec<AudioFile> {
        let mut shuffled = self.files.clone();
        shuffled.shuffle(&mut rng());
        shuffled
    }

}

pub fn play_audio_file_with_interrupt(path: &PathBuf) -> Result<PlaybackControl, Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Arc::new(Sink::try_new(&stream_handle)?);
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?
        .take_duration(Duration::from_secs(10));

    sink.append(source);
    sink.play();

    let sink_clone = Arc::clone(&sink);
    let running = Arc::new(AtomicBool::new(true));
    let r_flag = Arc::clone(&running);

    // Thread pour surveiller le timer
    let timer_handle = thread::spawn(move || {
        for remaining in (1..=10).rev() {
            if !r_flag.load(Ordering::SeqCst) {
                return;
            }
            print!("\r⏱️  Temps restant : {}s ", remaining);
            stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Lecture de touches
    let control = loop {
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('n') => break PlaybackControl::Next,
                    KeyCode::Char('s') => break PlaybackControl::Stop,
                    _ => {}
                }
            }
        }

        if sink_clone.empty() {
            break PlaybackControl::Timeout;
        }
    };

    running.store(false, Ordering::SeqCst); // Stop le timer proprement
    sink.stop();
    timer_handle.join().ok();

    // Efface ligne terminal
    stdout().execute(cursor::MoveToColumn(0))?;
    stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

    Ok(control)
}

pub fn play_audio_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?
        .take_duration(Duration::from_secs(10));

    sink.append(source);

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

    for remaining in (1..=10).rev() {
        stdout.execute(cursor::MoveToColumn(0))?;
        stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        write!(stdout, "⏱️  Temps restant : {}s", remaining)?;
        stdout.flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    sink.sleep_until_end();

    stdout.execute(cursor::MoveToColumn(0))?;
    stdout.execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
    println!("✅ Lecture terminée.");

    Ok(())
}

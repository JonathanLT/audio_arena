use std::path::PathBuf;
use walkdir::WalkDir;
use rand::seq::SliceRandom;
use rand::{rng};
use mp3_duration;

#[derive(Debug, Clone)]
pub struct AudioFile {
    pub path: PathBuf,
    pub duration: String, // Format "mm:ss"
}

pub struct AudioLibrary {
    pub files: Vec<AudioFile>,
}

impl AudioLibrary {
    /// Crée une nouvelle bibliothèque audio vide
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Charge tous les fichiers audio d'un dossier (récursif)
    pub fn load_from_dir(dir: &str) -> Self {
        let files = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let path = entry.into_path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext = ext.to_lowercase();
                    let duration = if ext == "mp3" {
                        // Utilisation de la crate mp3-duration pour obtenir la durée
                        match mp3_duration::from_path(&path) {
                            Ok(dur) => format!("{:02}:{:02}", dur.as_secs() / 60, dur.as_secs() % 60),
                            Err(_) => "00:00".to_string(),
                        }
                    } else {
                        "00:00".to_string() // Durée inconnue pour les autres formats
                    };
                    if ["mp3", "flac", "wav", "ogg", "m4a"].contains(&ext.as_str()) {
                        Some(AudioFile { path, duration })
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

    /// Retourne la liste des fichiers audio
    pub fn list(&self) -> &Vec<AudioFile> {
        &self.files
    }

    /// Retourne la liste mélangée des fichiers audio
    pub fn shuffled(&self) -> Vec<AudioFile> {
        let mut shuffled = self.files.clone();
        shuffled.shuffle(&mut rng());
        shuffled
    }
}
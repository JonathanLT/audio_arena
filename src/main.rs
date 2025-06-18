use audio_lib::AudioLibrary;
use audio_player::AudioPlayer;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let library = AudioLibrary::load_from_dir("musics");
    let files = library.shuffled();
    let player = AudioPlayer::new();

    println!("🎵 Bienvenue dans Audio Arena !");
    println!("Commandes : [Entrée] = suivant, p = pause, r = reprendre, s = stop, q = quitter\n");

    let mut is_paused = false;
    let mut index = 0;
    let mut file: &audio_lib::AudioFile;
    while !files.is_empty() {
        if !is_paused {
            file = files.get(index).expect("La bibliothèque audio est vide");
            println!("🎶 Lecture de : {:?}", file.path.file_name().unwrap());
            player.play(file.path.clone());
            index += 1;
        }

        loop {
            let mut buffer = [0; 1];
            if io::stdin().read(&mut buffer).is_ok() {
                match buffer[0] as char {
                    'p' => {
                        player.pause();
                        is_paused = true;
                        index -= 1; // Ne pas avancer si on met en pause
                        println!("⏸️ Pause");
                    }
                    'r' => {
                        player.resume();
                        is_paused = false;
                        println!("▶️ Reprise");
                    }
                    's' => {
                        player.stop();
                        println!("⛔ Arrêt demandé.");
                        return Ok(());
                    }
                    'q' => {
                        player.stop();
                        println!("👋 Quitter.");
                        return Ok(());
                    }
                    '\n' => {
                        player.stop();
                        break; // Passe à la musique suivante
                    }
                    _ => {}
                }
            }
        }
        if index >= files.len() {
            break;
        }
    }

    println!("✅ Fin de session.");
    Ok(())
}
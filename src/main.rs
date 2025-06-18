use audio_lib::{AudioLibrary, play_audio_file_with_interrupt, PlaybackControl};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let library = AudioLibrary::load_from_dir("musics");
    let files = library.shuffled();
    println!("🎵 Bienvenue dans Audio Arena ! Appuyez sur 'n' ou 's'\n");

    for (_i, file) in files.iter().enumerate() {
        println!("\n🎵 Lecture de : {:?}", file.path.file_name().unwrap());
        match play_audio_file_with_interrupt(&file.path) {
            Ok(PlaybackControl::Next) => continue,
            Ok(PlaybackControl::Timeout) => continue,
            Ok(PlaybackControl::Stop) => {
                println!("⛔ Arrêt demandé.");
                break;
            }
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }

    println!("✅ Fin de session.");

    Ok(())
}

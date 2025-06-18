use audio_lib::{AudioLibrary, AudioFile};
use audio_player::AudioPlayer;
use eframe::{egui, App, Frame};
use rand::rng;

pub struct GuiPlayerApp {
    playlist: Vec<AudioFile>,
    current: Option<AudioFile>,
    is_playing: bool,
    is_paused: bool,
    player: AudioPlayer,
}

impl Default for GuiPlayerApp {
    fn default() -> Self {
        let library = AudioLibrary::load_from_dir("musics");
        let playlist = library.shuffled();
        Self {
            playlist,
            current: None,
            is_playing: false,
            is_paused: false,
            player: AudioPlayer::new(),
        }
    }
}

impl App for GuiPlayerApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio Arena");

            if let Some(current) = &self.current {
                ui.label(format!(
                    "En cours : {}",
                    current
                        .path
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default()
                ));
            } else {
                ui.label("Aucune lecture");
            }

            ui.horizontal(|ui| {
                if ui.button("Pause").clicked() {
                    self.player.pause();
                    self.is_paused = true;
                }
                if ui.button("Reprendre").clicked() {
                    self.player.resume();
                    self.is_paused = false;
                }
                if ui.button("Suivant").clicked() {
                    if let Some(current) = &self.current {
                        if let Some(idx) = self.playlist.iter().position(|f| f.path == current.path) {
                            let next_idx = (idx + 1) % self.playlist.len();
                            if let Some(next_file) = self.playlist.get(next_idx) {
                                self.player.play(next_file.path.clone());
                                self.current = Some(next_file.clone());
                                self.is_playing = true;
                                self.is_paused = false;
                            }
                        }
                    }
                }
                if ui.button("Stop").clicked() {
                    self.player.stop();
                    self.is_playing = false;
                    self.is_paused = false;
                    self.current = None;
                }
                if ui.button("Mélanger").clicked() {
                    use rand::seq::SliceRandom;
                    let mut v = self.playlist.clone();
                    v.shuffle(&mut rng());
                    self.playlist = v;
                }
            });

            ui.separator();
            ui.label("Playlist :");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in &self.playlist {
                    ui.horizontal(|ui| {
                        ui.label(
                            file.path
                                .file_name()
                                .map(|n| n.to_string_lossy())
                                .unwrap_or_default(),
                        );
                        if ui.button("Lire").clicked() {
                            self.player.play(file.path.clone());
                            self.current = Some(file.clone());
                            self.is_playing = true;
                            self.is_paused = false;
                        }
                    });
                }
            });
        });
    }
}

pub fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Audio Arena",
        options,
        Box::new(|_cc| Ok(Box::new(GuiPlayerApp::default()))),
    )
}
use audio_lib::{AudioLibrary, AudioFile};
use audio_player::AudioPlayer;
use eframe::{egui, App, Frame};
use egui::{TextStyle, TextWrapMode};
use rand::rng;
use rfd::FileDialog;
use log::{debug, error, info, trace, warn};
use env_logger::Env;
mod playlist_table;
use playlist_table::{Playlist, Track};
use std::collections::BTreeMap;
use egui::FontData;
use eframe::egui::{FontDefinitions, FontFamily};
use std::sync::Arc;

pub struct GuiPlayerApp {
    library: AudioLibrary,
    playlist: Vec<AudioFile>,
    current: Option<AudioFile>,
    is_playing: bool,
    is_paused: bool,
    player: AudioPlayer,
    picked_folder: Option<String>,
    folder_loaded: bool,
    playlist_table: Playlist,
}

impl GuiPlayerApp {
    fn setup_fonts(&mut self, ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        // Ajouter une nouvelle police
        fonts.font_data.insert(
            "ma_police".to_string(),
            Arc::new(FontData::from_static(include_bytes!("../fonts/M_PLUS_Rounded_1c/MPLUSRounded1c-Regular.ttf"))),
        );

        // Utiliser cette police comme principale pour le texte proportionnel
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "ma_police".to_string());

        ctx.set_fonts(fonts);
    }
}

impl Default for GuiPlayerApp {
    
    fn default() -> Self {
        Self {
            library: AudioLibrary::new(),
            playlist: Vec::new(),
            current: None,
            is_playing: false,
            is_paused: false,
            player: AudioPlayer::new(),
            picked_folder: None,
            folder_loaded: false,
            playlist_table: Playlist::new(Vec::new()),
        }
    }
}

fn main() -> eframe::Result {
    // Initialize the logger to capture debug messages
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 400.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Audio Arena",
        options,
        Box::new(|cc| {
            let mut app = GuiPlayerApp::default();
            app.setup_fonts(&cc.egui_ctx); // configuration de la police
            Ok(Box::new(app))
        }),
    )
}

impl eframe::App for GuiPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Open folder:");

                if ui.button("…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        info!("Picked folder: {}", path.display());
                        self.folder_loaded = true;
                        self.picked_folder = Some(path.display().to_string());
                        self.library = AudioLibrary::load_from_dir(path.to_str().unwrap());
                        self.playlist = self.library.shuffled();
                        self.playlist_table.clear();
                        for file in &self.playlist {
                            self.playlist_table.add_track(Track {
                                name: file.path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default().to_string(),
                                duration: file.duration.to_string(),
                            });
                        }
                    }
                }
                ui.separator();
                if let Some(picked_folder) = &self.picked_folder {
                    ui.horizontal(|ui| {
                        ui.label("Picked folder:");
                        ui.monospace(picked_folder);
                    });
                }
                if ui.button("Toggle hide").clicked() {
                    self.playlist_table.toggle_hidden();
                }
            });
            ui.separator();

            // lire la premiere piste de la playlist
            ui.horizontal(|ui| {
                if self.is_playing {
                    if ui.button("Previous").clicked() {
                        if let Some(current) = &self.current {
                            if let Some(idx) = self.playlist.iter().position(|f| f.path == current.path) {
                                let prev_idx = if idx == 0 { self.playlist.len() - 1 } else { idx - 1 };
                                if let Some(prev_file) = self.playlist.get(prev_idx) {
                                    self.playlist_table.set_current(prev_idx);
                                    self.player.play(prev_file.path.clone());
                                    self.current = Some(prev_file.clone());
                                    self.is_playing = true;
                                    self.is_paused = false;
                                }
                            }
                        }
                    }
                    if ui.button("Next").clicked() {
                        if let Some(current) = &self.current {
                            if let Some(idx) = self.playlist.iter().position(|f| f.path == current.path) {
                                let next_idx = (idx + 1) % self.playlist.len();
                                if let Some(next_file) = self.playlist.get(next_idx) {
                                    self.playlist_table.set_current(next_idx);
                                    self.player.play(next_file.path.clone());
                                    self.current = Some(next_file.clone());
                                    self.is_playing = true;
                                    self.is_paused = false;
                                }
                            }
                        }
                    }
                    if self.is_paused {
                        if ui.button("Reprendre").clicked() {
                            self.player.resume();
                            self.is_paused = false;
                        }
                    } else {
                        if ui.button("Pause").clicked() {
                            self.player.pause();
                            self.is_paused = true;
                        }
                    }
                    if ui.button("Stop").clicked() {
                        self.player.stop();
                        self.is_playing = false;
                        self.is_paused = false;
                        self.current = None;
                    }
                } else {
                    if ui.button("Play").clicked() {
                        if let Some(first_file) = self.playlist.first() {
                            self.playlist_table.set_current(0);
                            self.player.play(first_file.path.clone());
                            self.current = Some(first_file.clone());
                            self.is_playing = true;
                            self.is_paused = false;
                        } else {
                            info!("Aucune piste dans la playlist.");
                        }
                    }
                }
                if self.folder_loaded {
                    if ui.button("Shuffle").clicked() {
                        info!("Playlist shuffled.");
                        self.player.stop();
                        self.is_playing = false;
                        self.is_paused = false;
                        self.current = None;                        
                        self.playlist = self.library.shuffled();
                        self.playlist_table.clear();
                        for file in &self.playlist {
                            self.playlist_table.add_track(Track {
                                name: file.path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default().to_string(),
                                duration: file.duration.to_string(),
                            });
                        }
                    }
                    if ui.button("Clear Playlist").clicked() {
                        info!("Playlist cleared.");
                        self.player.stop();
                        self.playlist.clear();
                        self.playlist_table.clear();
                        self.current = None;
                        self.is_playing = false;
                        self.is_paused = false;
                        self.folder_loaded = false;
                    }
                }
            });
            ui.separator();

            // Display the current playlist
            ui.heading("Playlist:");
            self.playlist_table.show(ui);

        });
    }
}
use egui::{self, FontFamily, FontId, RichText, Ui};
use egui_extras::{Column, TableBuilder};

pub struct Track {
    pub name: String,
    pub duration: String, // Format "mm:ss"
}

pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current_index: Option<usize>, // Index du morceau en cours de lecture
}

impl Playlist {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self { tracks, current_index: Some(0) }
    }

    /// Ajoute une piste à la playlist
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Supprime une piste de la playlist par son index
    pub fn remove_track(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.tracks.remove(index);
        }
    }

    /// Définit le morceau en cours de lecture
    pub fn set_current(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current_index = Some(index);
        } else {
            self.current_index = None;
        }
    }

    /// Vide la playlist
    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn show(&self, ui: &mut Ui) {
        let available_height = ui.available_height();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::remainder()
                .at_most(30.0)
                .clip(true)
                .resizable(false)
            )
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .header(20.0, |mut ui| {
                ui.col(|ui| {
                    ui.strong("#");
                });
                ui.col(|ui| {
                    ui.label("Nom");
                });
                ui.col(|ui| {
                    ui.label("Durée");
                });
            })
            .body(|mut ui| {
                for (i, track) in self.tracks.iter().enumerate() {
                    let is_current = Some(i) == self.current_index;
                    let index_text = RichText::new(format!("{}", i + 1)).strong().color(if is_current { egui::Color32::WHITE } else { egui::Color32::DARK_GRAY });
                    let name_text = RichText::new(&track.name).strong().color(if is_current { egui::Color32::WHITE } else { egui::Color32::DARK_GRAY });
                    let duration_text = RichText::new(&track.duration).strong().color(if is_current { egui::Color32::WHITE } else { egui::Color32::DARK_GRAY });

                    ui.row(20.0, |mut ui| {
                        ui.col(|ui| {
                            ui.label(index_text);
                        });
                        ui.col(|ui| {
                            ui.label(name_text);
                        });
                        ui.col(|ui| {
                            ui.label(duration_text);
                        });
                    });
                }
            });
    }
}
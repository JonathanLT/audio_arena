use egui::{self, Ui, RichText};
use egui_extras::{Column, TableBuilder};

pub struct Track {
    pub name: String,
    pub duration: String, // Format "mm:ss"
}

pub struct Playlist {
    pub tracks: Vec<Track>,
    pub current_index: Option<usize>, // Index du morceau en cours de lecture
    pub hidden: bool,                 // Etat caché
}

impl Playlist {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            tracks,
            current_index: None,
            hidden: false,
        }
    }
    /// Ajoute une piste à la playlist
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Définit le morceau en cours de lecture
    pub fn set_current(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current_index = Some(index);
        } else {
            self.current_index = None;
        }
    }
    
    /// Affiche ou masque la playlist
    pub fn toggle_hidden(&mut self) {
        self.hidden = !self.hidden;
    }

    /// Vide la playlist
    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    /// Affiche la playlist. Retourne l'index cliqué si une ligne est sélectionnée.
    pub fn show(&self, ui: &mut Ui) -> Option<usize> {
        let mut clicked_index = Some(0);
        let available_height = ui.available_height();

        let mut table = TableBuilder::new(ui)
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
            });

            table.body(|mut ui| {
                for (i, track) in self.tracks.iter().enumerate() {
                    let is_current = Some(i) == self.current_index;
                    let index_text = RichText::new(format!("{}", i + 1)).strong().color(if is_current { egui::Color32::WHITE } else { egui::Color32::DARK_GRAY });
                    let name_text = RichText::new(if !self.hidden { &track.name } else { "**********************" }).strong().color(if is_current { egui::Color32::WHITE } else { egui::Color32::DARK_GRAY });
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

        clicked_index
    }
}
use eframe::egui;

struct Tab {
    name: String,
    file_text: String,
    word: String,
}

impl Tab {
    fn new(name: String, file_text: String) -> Self {
        Tab {
            name,
            file_text,
            word: String::new(),
        }
    }
}

struct MyApp {
    tabs: Vec<Tab>,
    selected: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            tabs: vec![Tab::new(
                String::from("Neue Tab"),
                String::new(),
            )],
            selected: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.painter().rect_filled(
            ui.max_rect(),
            0.0,
            egui::Color32::from_rgb(30, 30, 46),
        );
        ui.horizontal(|ui| {
            let mut tab_to_close: Option<usize> = None;

            for (i, tab) in self.tabs.iter().enumerate() {
                if ui.selectable_label(self.selected == i, &tab.name).clicked() {
                    self.selected = i;
                }
                if ui.button("x").clicked() {
                    tab_to_close = Some(i);
                }

                ui.separator();
            }

            if let Some(i) = tab_to_close {
                self.tabs.remove(i);
                if self.selected >= self.tabs.len() && !self.tabs.is_empty() {
                    self.selected = self.tabs.len() - 1;
                }
            }
            if ui.button("+").clicked() {
                let file = rfd::FileDialog::new()
                    .add_filter("text", &["txt"])
                    .pick_file();

                if let Some(path) = file {
                    let name = path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    match std::fs::read_to_string(&path) {
                        Ok(text) => {
                            self.tabs.push(Tab::new(name, text));
                            self.selected = self.tabs.len() - 1;
                        }
                        Err(e) => println!("{}", e),
                    }
                }
            }
        });

        ui.separator();
        if self.tabs.is_empty() {
            ui.label("Bitte Drucken sie den + Knopf!");
            return;
        }

        let tab = &mut self.tabs[self.selected];

        ui.text_edit_singleline(&mut tab.word);
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in tab.file_text.lines() {
                ui.horizontal(|ui| {
                    if tab.word.trim().is_empty() {
                        ui.label(line);
                    } else {
                        let word_lower = tab.word.to_lowercase();
                        let line_lower = line.to_lowercase();
                        let mut pos_in_line = 0;

                        while let Some(pos) = line_lower[pos_in_line..].find(&word_lower) {
                            let real_pos = pos_in_line + pos;

                            if real_pos > pos_in_line {
                                ui.label(&line[pos_in_line..real_pos]);
                            }

                            let match_end = real_pos + tab.word.len();
                            ui.label(
                                egui::RichText::new(&line[real_pos..match_end])
                                    .background_color(egui::Color32::YELLOW)
                                    .color(egui::Color32::BLACK),
                            );

                            pos_in_line = match_end;
                        }

                        if pos_in_line < line.len() {
                            ui.label(&line[pos_in_line..]);
                        }
                    }
                });
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Wort Sucher",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}
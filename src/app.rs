use eframe::App;
use egui::CollapsingHeader;
use egui_extras::{Column, TableBuilder};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

struct LsofOptions {
    regular: bool,
    directory: bool,
    block_special: bool,
    character_special: bool,
    socket: bool,
}

impl Default for LsofOptions {
    fn default() -> Self {
        Self {
            regular: true,
            directory: true,
            block_special: true,
            character_special: true,
            socket: true,
        }
    }
}

pub struct ShowOffApp {
    options: LsofOptions,
    open_files: OpenFilesWindow,
    tx: SyncSender<String>,
}

impl ShowOffApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        let (tx, rx) = sync_channel(1);

        Self {
            tx,
            options: LsofOptions::default(),
            open_files: OpenFilesWindow::new(rx),
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        let LsofOptions {
            regular,
            directory,
            block_special,
            character_special,
            socket,
        } = &mut self.options;

        ui.horizontal(|ui| {
            ui.checkbox(regular, "Regular");
            ui.checkbox(directory, "Directory");
            ui.checkbox(block_special, "Block Special");
            ui.checkbox(character_special, "Character Special");
            ui.checkbox(socket, "Socket");
        });

        ui.separator();
    }

    fn show_open_files(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.open_files.update(ctx, frame);
    }
}

impl eframe::App for ShowOffApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame)
            })
        });

        self.show_open_files(ctx, frame);
    }
}

struct OpenFilesWindow {
    rx: Receiver<String>,
}

impl eframe::App for OpenFilesWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            CollapsingHeader::new("Process1")
                .default_open(true)
                .show(ui, |ui| {
                    let table = TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::auto());

                    table.body(|mut body| {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("File1");
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("File2");
                            });
                        });
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label("File3");
                            });
                        });
                    })
                });

            CollapsingHeader::new("Process2")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("File1");
                });
        });
    }
}

impl OpenFilesWindow {
    fn new(rx: Receiver<String>) -> Self {
        Self { rx }
    }
}

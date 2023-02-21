use eframe::App;

pub struct LsofOptions {
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
    open_files: OpenFilesApp,
}

impl ShowOffApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            options: LsofOptions::default(),
            open_files: OpenFilesApp {},
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

pub struct OpenFilesApp {}

impl eframe::App for OpenFilesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Open Files");
        });
    }
}

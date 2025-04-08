use eframe::egui::{self, Label, RichText, Sense, TextEdit};


fn main() {
    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native("Yet Another ToDo",  native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}


#[derive(serde::Serialize, serde::Deserialize)]
struct Task {
    text: String,
    done: bool,
    edit: bool,
}

impl Task {
    fn default() -> Self {
        Task {
            text: String::from("New task"),
            done: false,
            edit: false,
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
struct Section {
    title: String,
    tasks: Vec<Task>,
    edit: bool,
}

impl Section {
    fn default() -> Self {
        Section {
            title: String::from("New Section"),
            tasks: vec![Task::default()],
            edit: true,
        }
    }

    fn add_task(&mut self, task: &str, edit: bool) {
        self.tasks.push(Task {text: task.to_string(), done: false, edit});
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
enum Mode {
    Idle,
    EditTask,
    EditSection,
}


#[derive(serde::Serialize, serde::Deserialize)]
struct MyApp {
    sections: Vec<Section>,
    mode: Mode,
    first_time_edit: bool,
    scale_factor: f32,
}


impl MyApp {
    fn default() -> Self {
        MyApp {
            sections: vec![Section::default()],
            mode: Mode::EditSection,
            first_time_edit: true,
            scale_factor: 1.0,
        }
    }


    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            if let Some(app) = eframe::get_value(storage, eframe::APP_KEY) {
                app
            } else {
                MyApp::default()
            }
        } else {
            MyApp::default()
        }
    }


    fn add_section(&mut self, title: &str, edit: bool) {
        self.sections.push(Section {title: title.to_string(), tasks: vec![], edit});
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                // In this mode you can click a Task or a Section to edit it,
                // the checkboxes to mark tasks as completed and add new Tasks
                // and Sections
                Mode::Idle => {
                    // Handle zooming
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        self.scale_factor += 0.2;

                        if self.scale_factor > 3.0 {
                            self.scale_factor = 3.0;
                        }

                        ctx.set_pixels_per_point(self.scale_factor);
                    }
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        self.scale_factor -= 0.2;

                        if self.scale_factor < 1.0 {
                            self.scale_factor = 1.0;
                        }

                        ctx.set_pixels_per_point(self.scale_factor);
                    }

                    for section in &mut self.sections {
                        // Render Section title as clickable, if clicked edit it
                        if ui.add(Label::new(RichText::new(&section.title).heading()).sense(Sense::click())).clicked() {
                            // Enter edit section mode
                            section.edit = true;
                            self.mode = Mode::EditSection;
                        }

                        // Render Tasks as clickable, if clicked edit it
                        for task in &mut section.tasks {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.done, "");
                                if ui.add(Label::new(&task.text).sense(Sense::click())).clicked() {
                                    task.edit = true;
                                    self.mode = Mode::EditTask;
                                    self.first_time_edit = true;
                                }
                            });
                        }

                        // Render an invisible Task used to add a Task
                        let response = ui.add(Label::new("                             "));
                        if response.clicked() {
                            let empty = String::new();
                            section.add_task(&empty, true);
                            self.mode = Mode::EditTask;
                            self.first_time_edit = true;
                        }
                    }

                    // Render an invisible Section used to add a Section
                    let response = ui.add(Label::new(RichText::new("                            ").heading()));
                    if response.clicked() {
                        let empty = String::new();
                        self.add_section(&empty, true);
                        self.mode = Mode::EditSection;
                        self.first_time_edit = true;
                    }
                },

                // In this mode all Sections and Tasks are rendered as plain
                // labels except for the Task being edited which should be an
                // edit box. This mode is also entered when a new task is added
                Mode::EditTask => {
                    for section in &mut self.sections {
                        ui.heading(&section.title);

                        for task in &mut section.tasks {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.done, "");
                                if task.edit {
                                    // Render edit text box for task
                                    let response = ui.add(TextEdit::singleline(&mut task.text));

                                    if self.first_time_edit {
                                        response.request_focus();
                                        self.first_time_edit = false;
                                    }

                                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                        self.mode = Mode::Idle;
                                        task.edit = false;
                                    }
                                } else {
                                    // Render normally
                                    ui.label(&task.text);
                                }
                            });
                        }

                        ui.add_space(12.0);
                    }
                },

                // In this mode all Sections and Tasks are rendered as plain
                // labels except for the Section being edited which should be an
                // edit box
                Mode::EditSection => {
                    for section in &mut self.sections {

                        if section.edit {
                            let response = ui.add(TextEdit::singleline(&mut section.title));

                            if self.first_time_edit {
                                response.request_focus();
                                self.first_time_edit = false;
                            }

                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)) {
                                self.mode = Mode::Idle;
                                section.edit = false;
                            }
                        } else {
                            ui.heading(&section.title);
                        }

                        for task in &mut section.tasks {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.done, "");
                                ui.label(&task.text);
                            });
                        }

                        ui.add_space(12.0);
                    }
                },
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

}

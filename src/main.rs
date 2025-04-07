use eframe::egui::{self, Label, RichText, Sense, TextEdit, ViewportBuilder};


fn main() {
    let mut native_options = eframe::NativeOptions::default();
    //let viewport = ViewportBuilder::default().with_inner_size([300.0, 600.0]);
    //native_options.viewport = viewport;

    let _ = eframe::run_native("Yet Another ToDo",  native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}


struct Task {
    text: String,
    done: bool,
    edit: bool,
}


struct Section {
    title: String,
    tasks: Vec<Task>,
    edit: bool,
}

impl Section {
    fn add_task(&mut self, task: &str, edit: bool) {
        self.tasks.push(Task {text: task.to_string(), done: false, edit});
    }
}


enum Mode {
    Idle,
    EditTask,
    EditSection,
}


struct MyApp {
    sections: Vec<Section>,
    mode: Mode,
    first_time_edit: bool,
    scale_factor: f32,
}


impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = MyApp {
            sections: vec![],
            mode: Mode::Idle,
            first_time_edit: false,
            scale_factor: 1.0,
        };

        app.add_section("Graphics", false);
        app.add_section("Gameplay", false);

        app.add_task("Graphics", "Main character sprite", false);
        app.add_task("Graphics", "Lab sprite", false);
        app.add_task("Graphics", "Brain sprite", false);

        app.add_task("Gameplay", "Figure out basic puzzle mechanic", false);

        app
    }


    fn add_section(&mut self, title: &str, edit: bool) {
        self.sections.push(Section {title: title.to_string(), tasks: vec![], edit});
    }

    fn add_task(&mut self, section_title: &str, task: &str, edit: bool) {
        for section in &mut self.sections {
            if section_title == section.title {
                section.add_task(task, edit);
                break;
            }
        }
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
}


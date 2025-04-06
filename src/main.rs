use eframe::egui::{self, Label, RichText, Sense, TextEdit, TextBuffer, ViewportBuilder};


fn main() {
    let mut native_options = eframe::NativeOptions::default();
    let viewport = ViewportBuilder::default().with_inner_size([300.0, 600.0]);
    native_options.viewport = viewport;

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


enum Mode {
    Idle,
    EditTask,
    EditSection,
    AddTask,
    AddSection,
}


struct MyApp {
    sections: Vec<Section>,
    mode: Mode,
}


impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = MyApp {
            sections: vec![],
            mode: Mode::Idle,
        };

        app.add_section("Graphics");
        app.add_section("Gameplay");

        app.add_task("Graphics", "Main character sprite");
        app.add_task("Graphics", "Lab sprite");
        app.add_task("Graphics", "Brain sprite");

        app.add_task("Gameplay", "Figure out basic puzzle mechanic");

        app
    }

    fn add_section(&mut self, title: &str) {
        self.sections.push(Section {title: title.to_string(), tasks: vec![], edit: false});
        println!("Added section {}", title);
    }

    fn add_task(&mut self, section_title: &str, task: &str) {
        for section in &mut self.sections {
            if section_title == section.title {
                println!("Added task {}", task);
                section.tasks.push(Task {text: task.to_string(), done: false, edit: false});
                break;
            } else {
                println!("Could not add task {}", task);
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
                    for section in &mut self.sections {
                        if ui.add(Label::new(RichText::new(&section.title).heading()).sense(Sense::click())).clicked() {
                            // Enter edit section mode
                            section.edit = true;
                            self.mode = Mode::EditSection;
                        }

                        for task in &mut section.tasks {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.done, "");
                                if ui.add(Label::new(&task.text).sense(Sense::click())).clicked() {
                                    task.edit = true;
                                    self.mode = Mode::EditTask;
                                }
                            });
                        }

                        ui.add_space(12.0);
                    }
                },

                // In this mode all Sections and Tasks are rendered as plain
                // labels except for the Task being edited which should be an
                // edit box
                Mode::EditTask => {
                    for section in &mut self.sections {
                        ui.heading(&section.title);

                        for task in &mut section.tasks {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.done, "");
                                if task.edit {
                                    // Render edit text box for task
                                    let response = ui.add(TextEdit::singleline(&mut task.text));

                                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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

                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
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

                Mode::AddTask => {
                },

                Mode::AddSection => {
                },
            }
        });
    }
}


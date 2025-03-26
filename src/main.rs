use eframe::egui;


fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Yet Another ToDo",  native_options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))));
}


struct Task {
    text: String,
    done: bool,
}


struct Section {
    title: String,
    tasks: Vec<Task>,
}


struct MyApp {
    sections: Vec<Section>,
}


impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = MyApp {
            sections: vec![],
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
        self.sections.push(Section {title: title.to_string(), tasks: vec![]});
        println!("Added section {}", title);
    }

    fn add_task(&mut self, section_title: &str, task: &str) {
        for section in &mut self.sections {
            if section_title == section.title {
                println!("Added task {}", task);
                section.tasks.push(Task {text: task.to_string(), done: false});
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

            for section in &mut self.sections {
                ui.heading(section.title.clone());

                for task in &mut section.tasks {
                    ui.checkbox(&mut task.done, task.text.clone());
                }
            }

        });
    }
}


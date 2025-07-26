use eframe::egui;

#[derive(Debug)]
pub struct MinimalDemo {
    counter: usize,
}

impl Default for MinimalDemo {
    fn default() -> Self {
        Self::new()
    }
}

impl MinimalDemo {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Minimal Context Menu Test - egui 0.32");
            ui.separator();

            ui.label("Try right-clicking on the button below:");
            ui.label("(The button is also a drag source)");
            ui.separator();

            // Single button that's both draggable and should have a context menu
            let button_id = egui::Id::new("test_button");

            let response = ui.dnd_drag_source(button_id, "drag_payload", |ui| {
                ui.button("Right-click me for context menu!")
            }).response;

            // Try to show context menu
            response.context_menu(|ui| {
                ui.label("Context Menu Works!");
                ui.separator();

                if ui.button("Increment Counter").clicked() {
                    self.counter += 1;
                }

                if ui.button("Reset Counter").clicked() {
                    self.counter = 0;
                }
            });

            ui.separator();
            ui.label(format!("Counter: {}", self.counter));
            ui.label("If context menu works, you should be able to increment this counter.");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Minimal Context Menu Test"),
        ..Default::default()
    };

    eframe::run_native(
        "Context Menu Test",
        options,
        Box::new(|_| Ok(Box::new(MinimalApp::new()))),
    )
}

struct MinimalApp {
    demo: MinimalDemo,
}

impl MinimalApp {
    fn new() -> Self {
        Self {
            demo: MinimalDemo::new(),
        }
    }
}

impl eframe::App for MinimalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo.show(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_initialization() {
        let demo = MinimalDemo::new();
        assert_eq!(demo.counter, 0);
    }
}
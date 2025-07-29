use eframe::egui;
use egui::Sense;

#[derive(Debug)]
enum ContextAction {
    ChangeColor(usize),
    Rename(usize),
    Duplicate(usize),
    Delete(usize),
}

#[derive(Clone, Debug)]
struct Item {
    name: String,
    color: egui::Color32,
}

impl Item {
    fn new(name: &str, color: egui::Color32) -> Self {
        Self {
            name: name.to_string(),
            color,
        }
    }
}

#[derive(Debug)]
pub struct DragDropDemo {
    items: Vec<Item>,
}

impl Default for DragDropDemo {
    fn default() -> Self {
        Self::new()
    }
}

impl DragDropDemo {
    pub fn new() -> Self {
        let items = vec![
            Item::new("Red Item", egui::Color32::RED),
            Item::new("Blue Item", egui::Color32::BLUE),
            Item::new("Green Item", egui::Color32::GREEN),
            Item::new("Yellow Item", egui::Color32::YELLOW),
        ];

        Self { items }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Drag & Drop with Context Menus");
            ui.separator();

            let mut swap_request: Option<(usize, usize)> = None;
            let mut context_action: Option<ContextAction> = None;

            // Display items in a simple layout
            ui.horizontal_wrapped(|ui| {
                for index in 0..self.items.len() {
                    let item = &self.items[index];
                    let response = self.show_draggable_item(ui, index, item);

                    // Handle drops
                    if let Some(dropped_index) = response.dnd_release_payload::<usize>() {
                        if *dropped_index != index {
                            swap_request = Some((*dropped_index, index));
                        }
                    }

                    // Add context menu
                    if let Some(action) = self.show_context_menu(response, index) {
                        context_action = Some(action);
                    }
                }
            });

            // Process swap request
            if let Some((from, to)) = swap_request {
                self.items.swap(from, to);
            }

            // Process context menu actions
            if let Some(action) = context_action {
                self.handle_context_action(action);
            }

            ui.separator();
            ui.label("Instructions:");
            ui.label("• Drag items to reorder them");
            ui.label("• Right-click for context menu");
        });
    }

    fn show_draggable_item(&self, ui: &mut egui::Ui, index: usize, item: &Item) -> egui::Response {
        let item_id = egui::Id::new(("item", index));

        ui.dnd_drag_source(item_id, index, |ui| {
            let button = egui::Button::new(&item.name)
                .fill(item.color)
                .min_size(egui::vec2(80.0, 40.0));

            ui.add(button)
        })
            .response
    }

    fn show_context_menu(
        &self,
        response: egui::Response,
        index: usize,
    ) -> Option<ContextAction> {
        let mut action = None;

        // The context_menu requires click sense to be opened on right-click. The response is
        // from a dnd_drag_source which internally only sets Sense::drag. So click needs to be added here.
        response.interact(Sense::click()).context_menu(|ui| {
            let item = &self.items[index];
            ui.label(format!("Item: {}", item.name));
            ui.separator();

            if ui.button("Change Color").clicked() {
                action = Some(ContextAction::ChangeColor(index));
            }

            if ui.button("Rename").clicked() {
                action = Some(ContextAction::Rename(index));
            }

            if ui.button("Duplicate").clicked() {
                action = Some(ContextAction::Duplicate(index));
            }

            if ui.button("Delete").clicked() {
                action = Some(ContextAction::Delete(index));
            }
        });

        action
    }

    fn handle_context_action(&mut self, action: ContextAction) {
        match action {
            ContextAction::ChangeColor(index) => {
                self.cycle_color(index);
            }
            ContextAction::Rename(index) => {
                let old_name = &self.items[index].name;
                self.items[index].name = format!("{} (Modified)", old_name);
            }
            ContextAction::Duplicate(index) => {
                let item = &self.items[index];
                let new_item = Item::new(&format!("{} Copy", item.name), item.color);
                self.items.push(new_item);
            }
            ContextAction::Delete(index) => {
                self.items.remove(index);
            }
        }
    }

    fn cycle_color(&mut self, index: usize) {
        let current_color = self.items[index].color;
        let new_color = match current_color {
            c if c == egui::Color32::RED => egui::Color32::BLUE,
            c if c == egui::Color32::BLUE => egui::Color32::GREEN,
            c if c == egui::Color32::GREEN => egui::Color32::YELLOW,
            c if c == egui::Color32::YELLOW => egui::Color32::PURPLE,
            _ => egui::Color32::RED,
        };
        self.items[index].color = new_color;
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Simple Drag & Drop Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Drag Drop Demo",
        options,
        Box::new(|_| Ok(Box::new(DragDropApp::new()))),
    )
}

struct DragDropApp {
    demo: DragDropDemo,
}

impl DragDropApp {
    fn new() -> Self {
        Self {
            demo: DragDropDemo::new(),
        }
    }
}

impl eframe::App for DragDropApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo.show(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("Test", egui::Color32::RED);
        assert_eq!(item.name, "Test");
        assert_eq!(item.color, egui::Color32::RED);
    }

    #[test]
    fn test_demo_initialization() {
        let demo = DragDropDemo::new();
        assert_eq!(demo.items.len(), 4);
        assert_eq!(demo.items[0].name, "Red Item");
    }
}
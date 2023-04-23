// This struct is used to store the renderer configuration, wich is : nodes_spacing, node_on_color, node_off_color
struct RenderConfig {
    nodes_spacing: egui::Vec2,
    node_size: egui::Vec2,
    node_on_color: egui::Color32,
    node_off_color: egui::Color32,
}
pub fn render_square_discrete<T: PartialOrd + std::ops::Neg<Output = T> + From<u32> + Copy>(
    ui: &mut egui::Ui,
    state: &mut Vec<T>,
    state_change_mask: &mut Vec<bool>,
    state_changed_falg: &mut bool,
    node_size: egui::Vec2,
    mouse_pos: egui::Pos2,
    mouse_down: bool,
    nodes_being_edited: &mut bool,
) {
    if state.is_empty() {
        panic!("Cannot render empty state");
    }

    if state.len() != state_change_mask.len() {
        panic!("State and state_change_mask are not the same length");
    }

    let state_sqrt = (state.len() as f32).sqrt() as usize;
    // This check, is done every frame, may be a good idea find a better way to check this
    if state_sqrt.pow(2) != state.len() {
        panic!("State is not a square");
    }

    // Confronting with the 0, we can cover both the case where the off-node is rapresented with 0 and the one where it is -1
    // To be abele to confront with T, we need to convert into it
    let zero = T::from(0);

    *state_changed_falg = false;

    // Main node where the rendering happens
    egui::Grid::new("central_panel_grid_0")
        .spacing(egui::vec2(3.0, 3.0))
        .min_col_width(0.0)
        .min_row_height(0.0)
        .show(ui, |ui| {
            for i in 0..state.len() {
                let mut button = egui::Button::new(" ")
                    .sense(egui::Sense::click())
                    .min_size(node_size);

                if state[i] <= zero {
                    button = button.fill(egui::Color32::from_rgb(0, 0, 0));
                } else {
                    button = button.fill(egui::Color32::from_rgb(255, 255, 255));
                }

                let response = ui.add(button);
                // If the mouse is over the button, and the mouse is pressed, invert it's state
                if response.rect.contains(mouse_pos) && mouse_down && !state_change_mask[i] {
                    *state_changed_falg = true;
                    state_change_mask[i] = true;
                    state[i] = -state[i];
                    *nodes_being_edited = true;
                }

                if (i + 1) % state_sqrt == 0 {
                    ui.end_row();
                }
            }
        });
}

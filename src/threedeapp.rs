use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};


#[derive(Default)]
pub struct ThreedeAppLayout {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    label: String,
}




fn bevy_draw_left_panel(app: &mut ThreedeAppLayout, ui: &mut egui::Ui) {
    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
}

fn bevy_draw_right_panel(app: &mut ThreedeAppLayout, ui: &mut egui::Ui) {
    ui.heading("Side Panel");

    ui.horizontal(|ui| {
        ui.label("Write something: ");
        ui.text_edit_singleline(&mut app.label);
    });

    ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
}

fn bevy_draw_gui(
    mut egui_context: ResMut<EguiContext>,
    mut appLayout: ResMut<ThreedeAppLayout>,
) {
    appLayout.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            bevy_draw_left_panel(&mut appLayout, ui)
        })
        .response
        .rect
        .width();
    appLayout.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            bevy_draw_right_panel(&mut appLayout, ui)
        })
        .response
        .rect
        .width();
}

fn bevy_update(threedeApp: ResMut<ThreedeAppLayout>)
{}


pub fn bevy_init(app: &mut App) {
    app.init_resource::<ThreedeAppLayout>();
    app.add_system(bevy_update);
    app.add_system(bevy_draw_gui);
}


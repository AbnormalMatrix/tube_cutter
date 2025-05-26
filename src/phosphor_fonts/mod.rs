pub mod variants;
pub use variants::*;

pub fn add_to_fonts(fonts: &mut nannou_egui::egui::FontDefinitions, variant: Variant) {
    fonts
        .font_data
        .insert("phosphor".into(), variant.font_data().into());

    if let Some(font_keys) = fonts.families.get_mut(&nannou_egui::egui::FontFamily::Proportional) {
        font_keys.insert(1, "phosphor".into());
    }
}
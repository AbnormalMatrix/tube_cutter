use nannou_egui::egui;

#[cfg(feature = "bold")]
pub mod bold;
#[cfg(feature = "fill")]
pub mod fill;
#[cfg(feature = "light")]
pub mod light;

pub mod regular;
#[cfg(feature = "thin")]
pub mod thin;

#[cfg(not(any(
    feature = "thin",
    feature = "light",
    feature = "regular",
    feature = "bold",
    feature = "fill",
)))]
// No compile_error needed; just enable "regular" by default in your Cargo.toml features section.
// Remove all #[cfg(feature = "...")] attributes except for "regular" if you want only regular enabled.
// The code below is not needed if "regular" is always enabled.

#[derive(Debug, Clone, Copy)]
pub enum Variant {
    #[cfg(feature = "thin")]
    Thin,
    #[cfg(feature = "light")]
    Light,
    Regular,
    #[cfg(feature = "bold")]
    Bold,
    #[cfg(feature = "fill")]
    Fill,
}

impl Variant {
    pub fn font_bytes(&self) -> &'static [u8] {
        match self {
            #[cfg(feature = "thin")]
            Variant::Thin => &*include_bytes!("../../res/Phosphor-Thin.ttf"),
            #[cfg(feature = "light")]
            Variant::Light => &*include_bytes!("../../res/Phosphor-Light.ttf"),
            Variant::Regular => &*include_bytes!("../../res/Phosphor.ttf"),
            #[cfg(feature = "bold")]
            Variant::Bold => &*include_bytes!("../../res/Phosphor-Bold.ttf"),
            #[cfg(feature = "fill")]
            Variant::Fill => &*include_bytes!("../../res/Phosphor-Fill.ttf"),
            #[allow(unreachable_patterns)]
            _ => panic!("This font variant is not enabled via features."),
        }
    }

    pub fn font_data(&self) -> egui::FontData {
        egui::FontData::from_static(self.font_bytes())
    }
}
use gtk4::gio::Settings;
use gtk4::pango;
use gtk4::pango::Weight;
use gtk4::prelude::{FontFamilyExt, SettingsExt};

const DEFAULT_FONT_FAMILY: &str = "sans";

pub fn find_available_font(families: &str, ctx: &pango::Context) -> String {
    let available_fonts: Vec<String> = ctx.list_families().iter().map(|f| f.name().to_ascii_lowercase()).collect();

    for font in families.split(',') {

        // System-ui is a special font that should be handled by the system.
        if font == "system-ui" {
            return get_system_ui_font();
        }

        println!("Checking for: {}", font);
        let font_name = font.trim().replace('"', "");

        if available_fonts.contains(&font_name.to_ascii_lowercase()) {
            return font_name;
        }
    }

    DEFAULT_FONT_FAMILY.to_string()
}

pub fn to_pango_weight(w: usize) -> Weight {
    if w < 100 {
        Weight::Thin
    } else if w < 200 {
        Weight::Ultralight
    } else if w < 300 {
        Weight::Light
    } else if w < 400 {
        Weight::Normal
    } else if w < 500 {
        Weight::Medium
    } else if w < 600 {
        Weight::Semibold
    } else if w < 700 {
        Weight::Bold
    } else if w < 800 {
        Weight::Ultrabold
    } else {
        Weight::Heavy
    }
}

/// Returns the font as defined by the gnome settings.
fn get_system_ui_font() -> String {
    let settings = Settings::new("org.gnome.desktop.interface");
    settings.string("font-name").to_string()
}
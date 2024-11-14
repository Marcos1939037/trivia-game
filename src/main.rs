use std::{fs::File, io::Read};

use eframe::egui;
use calabozos_y_preguntones::app::App;
use egui::FontDefinitions;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.,680.])
            .with_resizable(false)
            .with_maximize_button(false),
        centered: true,
        vsync: true,
        ..Default::default()
    };

    let mut fonts = egui::FontDefinitions::default();
    import_fonts(&mut fonts);

    let _ = eframe::run_native(
        "Calabozos y preguntones",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    );

    Ok(())
}

fn import_fonts(fonts: &mut FontDefinitions) {
    let mut font_data1 = Vec::new();
    File::open("assets/font/upheavtt.ttf")
        .expect("Error al abrir el archivo de fuente")
        .read_to_end(&mut font_data1)
        .expect("Error al leer el archivo de fuente");
    
    let mut font_data2 = Vec::new();
    File::open("assets/font/VCR_OSD_MONO_1.ttf")
        .expect("Error al abrir el archivo de fuente")
        .read_to_end(&mut font_data2)
        .expect("Error al leer el archivo de fuente");

    fonts.font_data.insert(
        "fuente_1".to_owned(),
        egui::FontData::from_owned(font_data1),
    );

    fonts.font_data.insert(
        "fuente_2".to_owned(),
        egui::FontData::from_owned(font_data2),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("CustomFont_1".into()))
        .or_default()
        .insert(0, "fuente_1".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "fuente_2".to_owned());
}

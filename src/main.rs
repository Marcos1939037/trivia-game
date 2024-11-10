use eframe::egui;
use calabozos_y_preguntones::app::App;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.,680.])
            .with_min_inner_size([1200.,680.]),
        centered: true,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Calabozos y preguntones",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    );

    Ok(())
}


use egui::{Color32, Ui};
use crate::app::App;

pub fn health_bar(ui: &mut Ui, health: f32, right_to_left: bool) {
  let (_, rect) = ui.allocate_space(egui::vec2(200.0, 25.0));

  ui.painter().rect_filled(
    rect,
    2.5,
    Color32::from_rgb(163, 43, 38)
  );

  let health_width = rect.width() * health;

  let health_rect = if right_to_left {
    // De derecha a izquierda
    egui::Rect::from_min_size(
      egui::pos2(rect.max.x - health_width, rect.min.y),
      egui::vec2(health_width, rect.height()),
    )
  } else {
    // De izquierda a derecha
    egui::Rect::from_min_size(
      rect.min,
      egui::vec2(health_width, rect.height()),
    )
  };

  ui.painter().rect_filled(
    health_rect,
    2.5,
    Color32::from_rgb(62, 148, 37)
  );
}

pub fn question_mode_1(ui: &mut Ui, app: &mut App) {
  ui.vertical_centered(|ui| {
    let button_size = egui::vec2(200.0, 50.0);
    let available_height = 250.0; // Espacio total disponible
    let total_buttons_height = 2.0 * button_size.y; // Espacio que necesitan los botones (4 botones * 50 altura cada uno)
    let spacing = (available_height - total_buttons_height) / 3.0;  // Espacio que queremos entre elementos
    ui.add_space(spacing);

    if ui.add_sized(button_size, egui::Button::new("Quitar vida hero")).clicked() {
      app.health.hero_health -= 0.05;
      app.health.hero_health = app.health.hero_health.clamp(0.0, 1.0);
      println!("hero: {}", app.health.hero_health);
    }
    ui.add_space(spacing);
    
    if ui.add_sized(button_size, egui::Button::new("Quitar vida enemy")).clicked() {
      app.health.enemy_health -= 0.05;
      app.health.enemy_health = app.health.enemy_health.clamp(0.0, 1.0);
      println!("hero: {}", app.health.enemy_health);
    }
    ui.add_space(spacing);
  });
}
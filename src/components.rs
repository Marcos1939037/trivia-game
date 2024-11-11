use egui::{Color32, RichText, Ui};
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
  let button_size = egui::vec2(250.0, 45.0);
  let num_of_answers = app.quiz.respuestas.len() as f32;
  let spacing = if num_of_answers == 2.0 {
    51.0
  } else {
    12.
  };
  let correct_ans = &app.quiz.respuesta_correcta;
  ui.vertical_centered(|ui| {
    ui.add_space(spacing);
    for (key, answer) in app.quiz.respuestas.iter() {
      if correct_ans == key {
        if ui.add_sized(button_size, egui::Button::new(RichText::new(answer).size(15.)).fill(Color32::DARK_GREEN)).clicked() {
          app.health.enemy_health -= 0.05;
          app.health.enemy_health = app.health.enemy_health.clamp(0.0, 1.0);
        }
      }else {
        if ui.add_sized(button_size, egui::Button::new(RichText::new(answer).size(15.))).clicked() {
          app.health.hero_health -= 0.05;
          app.health.hero_health = app.health.hero_health.clamp(0.0, 1.0);
        }
      }

      ui.add_space(spacing);
    }
  });
}
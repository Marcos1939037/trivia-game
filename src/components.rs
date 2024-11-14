use std::{thread, time::{Duration, Instant}};

use egui::{Color32, RichText, Ui};
use rand::Rng;
use crate::app::{App, StreakState};

const WHITE: Color32 = egui::Color32::WHITE;

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
  let num_of_answers = app.quiz.current_quiz.respuestas.len() as f32;
  let spacing = if num_of_answers == 2.0 {
    51.0
  } else {
    12.
  };
  let correct_ans = &app.quiz.current_quiz.respuesta_correcta.to_owned();
  let answers = app.quiz.current_quiz.respuestas.clone();

  ui.vertical_centered(|ui| {
    ui.add_space(spacing);
    for (key, answer) in answers.iter() {
      if correct_ans == key {
        let clicked = ui.add_enabled_ui(!app.rnd_animation.is_animating, |ui| {
          ui.add_sized(
            button_size,
            egui::Button::new(RichText::new(answer).size(15.0))
              .fill(Color32::DARK_GREEN)
          ).clicked()
        }).inner;
        if clicked {
          app.session_data.total_quiz += 1;
          app.session_data.correct_answers += 1;

          let (mut best_streak, mut current_streak) = app.session_data.win_streak;
          current_streak += 1;
          if current_streak >= best_streak {
            best_streak = current_streak;
          }

          app.session_data.win_streak.0 = best_streak;
          app.session_data.win_streak.1 = current_streak;
          app.rnd_animation.is_animating = true;
          app.rnd_animation.animation_start = Some(Instant::now());
        }
      }else {
        let clicked = ui.add_enabled_ui(!app.rnd_animation.is_animating, |ui| {
          ui.add_sized(
            button_size,
            egui::Button::new(RichText::new(answer).size(15.0))
          ).clicked()
        }).inner;
        if clicked {
          app.streak = StreakState::NoStreak;
          app.health.hero_health -= 0.1;
          app.health.hero_health = app.health.hero_health.clamp(0.0, 1.0);
          app.session_data.total_quiz += 1;
          app.session_data.wrong_answers += 1;
          app.session_data.win_streak.1 = 0;
          select_new_quiz(app);
        }
      }
      ui.add_space(spacing);
    }
  });
}

pub fn rand_num_animation(ui: &mut Ui, app: &mut App, ctx: &egui::Context) {
  let remaining = if app.rnd_animation.animation_start.unwrap().elapsed() >= app.rnd_animation.duration {
    Duration::from_secs(0)
  } else {
    app.rnd_animation.duration - app.rnd_animation.animation_start.unwrap().elapsed()
  };
  
  if remaining == Duration::from_secs(0) {
    match app.streak {
      StreakState::NoStreak => {
        app.health.enemy_health -= app.rnd_animation.rnd_number as f32/100.0;
      },
      StreakState::X2 => {
        app.health.enemy_health -= (app.rnd_animation.rnd_number as f32/100.0)*2.0;
      },
      StreakState::X3 => {
        app.health.enemy_health -= (app.rnd_animation.rnd_number as f32/100.0)*3.0;
      }
    }
    match app.session_data.win_streak.1 {
      streak if streak >= 3 && streak < 5 => app.streak = StreakState::X2,
      streak if streak >= 5 => app.streak = StreakState::X3,
      _ => ()
    }
    app.health.enemy_health = app.health.enemy_health.clamp(0.0, 1.0);
    app.rnd_animation.is_animating = false;
    app.rnd_animation.animation_start = None;
    if app.rnd_animation.rnd_number as u8 >= app.session_data.best_hit {
      app.session_data.best_hit = app.rnd_animation.rnd_number as u8;
    }
    select_new_quiz(app);
  }

  ui.add_space(20.0);
  ui.vertical_centered(|ui| {
    ui.label(egui::RichText::new("Lanzando el dado...")
      .size(20.)
      .color(WHITE));

    ui.add_space(10.0);

    if remaining > Duration::from_secs(1) {
      ui.label(egui::RichText::new(app.rnd_animation.rnd_number.to_string())
        .size(30.)
        .color(WHITE)
      );
    }

    if remaining <= Duration::from_secs(1) {
      match app.streak {
        StreakState::NoStreak => {
          ui.label(egui::RichText::new(app.rnd_animation.rnd_number.to_string())
            .size(30.)
            .color(WHITE)
          );
        },
        StreakState::X2 => {
          ui.horizontal(|ui| {
            ui.add_space(235.0);
        
            ui.vertical(|ui| {
              ui.label(egui::RichText::new(app.rnd_animation.rnd_number.to_string())
                .size(30.)
                .color(WHITE));
            });
        
            ui.vertical(|ui| {
              ui.label(egui::RichText::new("x2")
                .size(30.)
                .color(Color32::ORANGE));
            });
          });
        },
        StreakState::X3 => {
          ui.horizontal(|ui| {
            ui.add_space(235.0);
        
            ui.vertical(|ui| {
              ui.label(egui::RichText::new(app.rnd_animation.rnd_number.to_string())
                .size(30.)
                .color(WHITE));
            });
        
            ui.vertical(|ui| {
              ui.label(egui::RichText::new("x3")
                .size(30.)
                .color(Color32::RED));
            });
          });
        }
      }
    }
  });

  if remaining >= Duration::from_secs(1) {
    let mut rng = rand::thread_rng();
    app.rnd_animation.rnd_number = rng.gen_range(1..=10);
  }

  thread::sleep(Duration::from_millis(50));
  ctx.request_repaint();
}

pub fn timer(ui: &mut Ui, app: &mut App, remaining: Duration) {
  ui.vertical_centered(|ui| {
    let minutes = remaining.as_secs() /60;
    let seconds = remaining.as_secs() % 60;

    if remaining == Duration::from_secs(0) {
      app.health.hero_health -= 0.1;
      app.health.hero_health = app.health.hero_health.clamp(0.0, 1.0);
      select_new_quiz(app);
    }

    ui.heading(egui::RichText::new(format!("{:02}:{:02}",minutes,seconds))
      .size(60.)
      .color(WHITE)
    );
  });
}

pub fn select_new_quiz(app: &mut App) {
  let new_quiz = get_unused_quiz_index(app).unwrap_or(0);
  app.quiz.current_quiz = app.quiz.quiz_items.get(new_quiz)
    .unwrap()
    .to_owned();

  app.quiz.used_quiz_items[app.quiz.used_quiz_idx] = new_quiz as u8;
  app.quiz.used_quiz_idx += 1;

  app.quiz.duration = match app.quiz.current_quiz.tipo_reactivo.as_str() {
    "Opción Múltiple" => Duration::from_secs(31),
    "Verdadero o Falso" => Duration::from_secs(16),
    _ => Duration::from_secs(0)
  };
  app.quiz.start_time = Instant::now();

  if app.quiz.used_quiz_idx >= app.quiz.used_quiz_items.len() {
    app.quiz.used_quiz_items = [0; 40];
    app.quiz.used_quiz_idx = 0;
  }
}

fn get_unused_quiz_index(app: &App) -> Option<usize> {
  let available_indices: Vec<usize> = (0..app.quiz.quiz_items.len())
    .filter(|&index| !app.quiz.used_quiz_items.contains(&(index as u8)))
    .collect();

  if available_indices.is_empty() {
    return None;
  }

  let mut rng = rand::thread_rng();
  let random_index = available_indices[rng.gen_range(0..available_indices.len())];
  
  Some(random_index)
}
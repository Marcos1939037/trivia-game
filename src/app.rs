use crate::components;
use std::{collections::HashMap, thread, time::{Duration, Instant}};
use egui::{Align, CentralPanel, Color32, Image, Layout, RichText, SidePanel, TopBottomPanel};
use rand::Rng;
use serde::{Deserialize, Serialize};

const WHITE: Color32 = egui::Color32::WHITE;

pub struct App {
  screen: CurrentScreen,
  pub quiz: Quiz,
  pub health: HealthStatus,
  pub session_data: AnalysisData,
  pub rnd_animation: RndNumberAnimation,
  pub streak: StreakState
}

pub enum StreakState {
  X2,
  X3,
  NoStreak
}

impl Default for StreakState {
  fn default() -> Self {
    Self::NoStreak
  }
}

pub struct RndNumberAnimation {
  pub animation_start: Option<Instant>,
  duration: Duration,
  pub is_animating: bool,
  pub rnd_number: usize,
}

impl Default for RndNumberAnimation {
  fn default() -> Self {
    RndNumberAnimation {
      animation_start: None,
      duration: Duration::from_secs(2),
      is_animating: false,
      rnd_number: 0,
    }
  }
}

pub struct AnalysisData {
  pub correct_answers: u8,
  pub wrong_answers: u8,
  pub win_streak: (u8, u8), // (best streak, current streak)
  pub total_quiz: u8,
  pub best_hit: u8
}

impl AnalysisData {
  pub fn get_hit_percentage(&self) -> f32 {
    (self.correct_answers as f32 / self.total_quiz as f32) * 100.0
  }
}

impl Default for AnalysisData {
  fn default() -> Self {
    AnalysisData {
      correct_answers: 0,
      wrong_answers: 0,
      win_streak: (0, 0),
      total_quiz: 0,
      best_hit: 0
    }
  }
}

pub struct Quiz {
  pub quiz_items: Vec<QuizItem>,
  pub current_quiz: QuizItem,
  pub used_quiz_items: [u8; 40],
  pub used_quiz_idx: usize,
  pub duration: Duration,
  pub start_time: Instant,
}

impl Default for Quiz {
  fn default() -> Self {
    let json_str = std::fs::read_to_string("assets/data/questions.json").unwrap();
    let quiz_items: Vec<QuizItem> = serde_json::from_str(&json_str).unwrap();
    let rng = rand::thread_rng().gen_range(0..quiz_items.len());
    let quiz = quiz_items[rng].clone();
    let used_quiz_items: [u8; 40] = [rng as u8; 40];
    let duration = match quiz.tipo_reactivo.as_str() {
      "Opción Múltiple" => Duration::from_secs(31),
      "Verdadero o Falso" => Duration::from_secs(16),
      _ => Duration::from_secs(0)
    };

    Quiz {
      quiz_items: quiz_items,
      current_quiz: quiz,
      used_quiz_items: used_quiz_items,
      used_quiz_idx: 1,
      duration,
      start_time: Instant::now(),
    }
  }
}

pub struct HealthStatus {
  pub enemy_health: f32,
  pub hero_health: f32,
}

impl Default for HealthStatus {
  fn default() -> Self {
    HealthStatus {
      enemy_health: 1.0, 
      hero_health: 1.0
    }
  }
}

enum CurrentScreen {
  Menu,
  Ingame,
  Analisis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizItem {
  #[serde(rename = "Unidad Temática")]
  unidad_tematica: String,
  
  #[serde(rename = "Pregunta")]
  pregunta: String,
  
  #[serde(rename = "Respuestas")]
  pub respuestas: HashMap<String, String>,
  
  #[serde(rename = "Respuesta correcta")]
  pub respuesta_correcta: String,
  
  #[serde(rename = "Tipo de reactivo")]
  pub tipo_reactivo: String,
}

impl Clone for QuizItem {
  fn clone(&self) -> Self {
    QuizItem {
      unidad_tematica: self.unidad_tematica.clone(),
      pregunta: self.pregunta.clone(),
      respuestas: self.respuestas.clone(),
      respuesta_correcta: self.respuesta_correcta.clone(),
      tipo_reactivo: self.tipo_reactivo.clone()
    }
  }
}

impl App {
  pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    Self {
      screen: CurrentScreen::Menu,
      quiz: Quiz::default(),
      health: HealthStatus::default(),
      session_data: AnalysisData::default(),
      rnd_animation: RndNumberAnimation::default(),
      streak: StreakState::default(),
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    match self.screen {
      CurrentScreen::Menu => menu_ui(self, ctx),
      CurrentScreen::Ingame => ingame_ui(self, ctx),
      CurrentScreen::Analisis => analisis_ui(self, ctx),
    }
    if !self.rnd_animation.is_animating {
      ctx.request_repaint_after(Duration::from_millis(250));
    }
  }
}

fn menu_ui(app: &mut App, ctx: &egui::Context) {
  SidePanel::left("left_panel_menu")
    .min_width(600.0)
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
    ui.vertical_centered(|ui| {
      ui.add(
        Image::new(egui::include_image!("../assets/img/logo.png"))
        .max_width(200.0)
        .max_height(200.0)
      );
      ui.label(RichText::new("CALABOZOS Y PREGUNTONES")
        .family(egui::FontFamily::Name("CustomFont_1".into()))
        .size(60.0)
        .color(WHITE)
      );
    })
  });
  
  CentralPanel::default().show(ctx, |ui| {
    ui.centered_and_justified(|ui| {
      ui.vertical_centered(|ui| {
        if ui.add_sized(egui::vec2(200.0, 50.0), egui::Button::new("Iniciar")).clicked() {
          app.screen = CurrentScreen::Ingame;
        }
      });
    });
  });
}

fn ingame_ui(app: &mut App, ctx: &egui::Context) {
  // Actualizar el tiempo restante si el timer está corriendo
  let remaining = if app.quiz.start_time.elapsed() >= app.quiz.duration {
    Duration::from_secs(0)
  } else {
    app.quiz.duration - app.quiz.start_time.elapsed()
  };
  if app.health.enemy_health == 0.0 || app.health.hero_health == 0.0 {
    app.screen = CurrentScreen::Analisis;
  };

  TopBottomPanel::top("top_panel_ingame")
  .min_height(15.)
  .resizable(false)
  .show_separator_line(false)
  .show(ctx, |ui| {
    ui.horizontal(|ui| {
      ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.label(
          RichText::new(&app.quiz.current_quiz.unidad_tematica)
            .size(15.0)
        );
        ui.add_space(ui.available_width() - 60.);            
        if ui.add_sized(egui::vec2(25.0, 10.0), egui::Button::new("☰ Menu")).clicked() {
          println!("Botón clicado!");
        }
      });
    });
    ui.separator();
  });

  TopBottomPanel::bottom("bottom_panel_ingame")
    .min_height(250.0)
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      ui.separator();
      components::question_mode_1(ui, app);
  });

  SidePanel::left("left_panel_ingame")
    .min_width(350.0)
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      ui.add_space(5.
      );
      ui.vertical_centered(|ui| {
        components::health_bar(ui, app.health.hero_health, false);
        ui.add_space(120.0);
        ui.add(
          Image::new(egui::include_image!("../assets/img/hero.png"))
          .max_width(180.0)
          .max_height(180.0)
        );
      });
  });
    
  SidePanel::right("right_panel_ingame")
    .min_width(350.0)
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      ui.add_space(5.);
      ui.vertical_centered(|ui| {
        components::health_bar(ui, app.health.enemy_health, true);
        ui.add_space(150.0);
        ui.add(
          Image::new(egui::include_image!("../assets/img/enemy_1.png"))
          .max_width(150.0)
          .max_height(150.0)
        );
      });
  });

  CentralPanel::default().show(ctx, |ui| {    
    if app.rnd_animation.is_animating {
      let remaining = if app.rnd_animation.animation_start.unwrap().elapsed() >= app.rnd_animation.duration {
        Duration::from_secs(0)
      } else {
        app.rnd_animation.duration - app.rnd_animation.animation_start.unwrap().elapsed()
      };
      
      if remaining == Duration::from_secs(0) {
        app.health.enemy_health -= app.rnd_animation.rnd_number as f32/10.0;
        app.health.enemy_health = app.health.enemy_health.clamp(0.0, 1.0);
        app.rnd_animation.is_animating = false;
        app.rnd_animation.animation_start = None;
        if app.rnd_animation.rnd_number as u8 >= app.session_data.best_hit {
          app.session_data.best_hit = app.rnd_animation.rnd_number as u8;
        }
        components::select_new_quiz(app);
      }

      ui.add_space(20.0);
      ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("Lanzando el dado...")
        .size(20.)
        .color(WHITE));

      ui.add_space(10.0);

      ui.label(egui::RichText::new(app.rnd_animation.rnd_number.to_string())
        .size(25.)
        .color(WHITE));
      });

      if remaining >=Duration::from_secs(1) {
        let mut rng = rand::thread_rng();
        app.rnd_animation.rnd_number = rng.gen_range(1..=10);
      }
      thread::sleep(Duration::from_millis(50));
      ctx.request_repaint();
    }else {
      components::timer(ui, app, remaining);
      
      ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.label(egui::RichText::new(&app.quiz.current_quiz.pregunta)
        .size(30.)
        .color(WHITE));
      });
    }
  });
}


fn analisis_ui(app: &mut App, ctx: &egui::Context) {
  SidePanel::left("left_results_panel")
    .resizable(false)
    .exact_width(600.0)
    .show(ctx, |ui| {
      ui.centered_and_justified(|ui| {
        ui.label(RichText::new("GANASTE?\nPERDISTE?").family(egui::FontFamily::Name("CustomFont_1".into())).size(50.0))
      })
  });
  CentralPanel::default().show(ctx, |ui| {
    let lost_health = 1.0 - app.health.hero_health;
    let lost_health = (lost_health * 100.0) as u8;

    let hit_percentage = app.session_data.get_hit_percentage().floor();

      ui.add_space(20.0);
      ui.vertical_centered(|ui| {
        ui.label(RichText::new("Resultados").size(40.0))
      });
      ui.add_space(100.0);
      egui::Grid::new("results_table")
      .min_col_width(300.0)
      .spacing([40.0, 25.0])
      .show(ui, |ui| {
        ui.vertical_centered(|ui| {ui.label(RichText::new("Cantidad de aciertos").size(18.0))});
        ui.label(RichText::new(app.session_data.correct_answers.to_string()).size(18.0));
        ui.end_row();
  
        ui.vertical_centered(|ui| {ui.label(RichText::new("Numero de respuestas erroneas").size(18.0))});
        ui.label(RichText::new(app.session_data.wrong_answers.to_string()).size(18.0));
        ui.end_row();
        
        ui.vertical_centered(|ui| {ui.label(RichText::new("Procentaje de aciertos").size(18.0))});
        ui.label(RichText::new(format!("{}%",hit_percentage.to_string())).size(18.0));
        ui.end_row();
        
        ui.vertical_centered(|ui| {ui.label(RichText::new("Mejor racha de aciertos").size(18.0))});
        ui.label(RichText::new(app.session_data.win_streak.0.to_string()).size(18.0));
        ui.end_row();
        
        ui.vertical_centered(|ui| {ui.label(RichText::new("Mayor daño inflingido").size(18.0))});
        ui.label(RichText::new(app.session_data.best_hit.to_string()).size(18.0));
        ui.end_row();
        
        ui.vertical_centered(|ui| {ui.label(RichText::new("Vida total perdida").size(18.0))});
        ui.label(RichText::new(format!("{}%",lost_health.to_string())).size(18.0));
        ui.end_row();
      });
  });
}
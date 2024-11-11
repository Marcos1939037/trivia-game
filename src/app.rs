use crate::components;
use std::{collections::HashMap, time::{Duration, Instant}};
use egui::{Align, CentralPanel, Color32, Image, Layout, RichText, SidePanel, TopBottomPanel};
use rand::Rng;
use serde::{Deserialize, Serialize};

const WHITE: Color32 = egui::Color32::WHITE;

pub struct App {
  quiz_items: Vec<QuizItem>,
  pub quiz: QuizItem,
  used_quiz_items: [u8; 40],
  used_quiz_idx: usize,
  screen: CurrentScreen,
  duration: Duration,
  start_time: Instant,
  pub health: HealthStatus,
  refresh: u64
}

pub struct HealthStatus {
  pub enemy_health: f32,
  pub hero_health: f32,
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
  tipo_reactivo: String,
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

    let json_str = std::fs::read_to_string("assets/data/questions.json").unwrap();
    let quiz_items: Vec<QuizItem> = serde_json::from_str(&json_str).unwrap();

    let rng = rand::thread_rng().gen_range(0..quiz_items.len());
    let quiz = quiz_items[rng].clone();
    let used_quiz_items: [u8; 40] = [rng as u8; 40];

    // for item in &quiz_items {
    //   println!("Pregunta: {}", item.pregunta);
    //   println!("Tipo: {}", item.tipo_reactivo);
    //   println!("Respuesta correcta: {}", item.respuesta_correcta);
    //   println!("Opciones:");
    //   for (key, value) in &item.respuestas {
    //     println!("  {}: {}", key, value);
    //   }
    // }

    Self {
      quiz_items: quiz_items,
      quiz: quiz,
      used_quiz_items: used_quiz_items,
      used_quiz_idx: 1,
      screen: CurrentScreen::Menu,
      duration: Duration::from_secs(60),
      start_time: Instant::now(),
      health: HealthStatus {
        enemy_health: 1.0,
        hero_health: 1.0
      },
      refresh: 0
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
    println!("{:?}", self.used_quiz_items);
    // self.refresh += 1;
    // println!("{}",self.refresh);
    ctx.request_repaint_after(Duration::from_millis(250));
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
    let remaining = if app.start_time.elapsed() >= app.duration {
      Duration::from_secs(0)
    } else {
      app.duration - app.start_time.elapsed()
    };

    if app.health.enemy_health == 0.0 || app.health.hero_health == 0.0 {
      let new_quiz = get_unused_quiz_index(app).unwrap_or(0);
      app.quiz = app.quiz_items.get(new_quiz)
          .unwrap()
          .to_owned();

      
      app.used_quiz_items[app.used_quiz_idx] = new_quiz as u8;
      app.used_quiz_idx += 1;
      app.health.hero_health = 1.0;
      app.health.enemy_health = 1.0;

      if app.used_quiz_idx >= app.used_quiz_items.len() {
        app.used_quiz_items = [0; 40];
        app.used_quiz_idx = 0;
      }
    }

    TopBottomPanel::top("top_panel_ingame")
    .min_height(15.)
    .resizable(false)
    .show_separator_line(false)
    .show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
          ui.label(
            RichText::new("Unidad tematica 1 - Fundamentos de simulación y modelación")
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
        ui.add_space(5.);
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
      let minutes = remaining.as_secs() /60;
      let seconds = remaining.as_secs() % 60;

      ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new(format!("{:02}:{:02}",minutes,seconds))
          .size(60.)
          .color(WHITE)
        );
      });
      ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.label(egui::RichText::new(&app.quiz.pregunta)
        .size(30.)
        .color(WHITE));
      });
    });
}

#[allow(unused)]
fn analisis_ui(app: &mut App, ctx: &egui::Context) {
  todo!()
}

fn get_unused_quiz_index(app: &App) -> Option<usize> {
  let available_indices: Vec<usize> = (0..app.quiz_items.len())
      .filter(|&index| !app.used_quiz_items.contains(&(index as u8)))
      .collect();

  if available_indices.is_empty() {
      return None;
  }

  let mut rng = rand::thread_rng();
  let random_index = available_indices[rng.gen_range(0..available_indices.len())];
  
  Some(random_index)
}
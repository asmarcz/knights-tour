use std::{
    cmp,
    sync::mpsc::{self, Receiver, TryRecvError},
};

use eframe::{
    egui::{self},
    NativeOptions,
};
use solver::{solve, Pos};

mod solver;

struct App {
    curr_sol: Option<usize>,
    dimensions: Pos,
    receiver: Option<Receiver<Vec<Pos>>>,
    solutions: Vec<Vec<Pos>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            curr_sol: None,
            dimensions: Pos { x: 5, y: 5 },
            receiver: None,
            solutions: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Some(ref rx) = self.receiver {
            match rx.try_recv() {
                Ok(solution) => self.solutions.push(solution),
                Err(TryRecvError::Disconnected) => {
                    self.receiver = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }

        egui::SidePanel::left("results_panel").show(ctx, |ui| {
            ui.style_mut().text_styles = [
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    let width = ui.available_width() * 0.9;
                    for i in 0..self.solutions.len() {
                        let btn = egui::Button::new(format!("Solution {}", i))
                            .min_size(egui::Vec2 { x: width, y: 0.0 })
                            .selected(self.curr_sol.is_some_and(|x| x == i));
                        if ui.add(btn).clicked() {
                            self.curr_sol = Some(i);
                        }
                    }
                });
            ui.allocate_space(ui.available_size());
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let grid_width = ui.available_width();
            let grid_height = ui.available_height();
            let rect_side = if grid_width > grid_height {
                grid_height
            } else {
                grid_width
            } / cmp::max(self.dimensions.x, self.dimensions.y) as f32;
            let button_size = egui::Vec2 {
                x: rect_side,
                y: rect_side,
            };
            let dark_color = egui::Color32::from_rgb(181, 136, 99);
            let light_color = egui::Color32::from_rgb(240, 217, 181);

            ui.style_mut().text_styles = [
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(32.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::BLACK);

            egui::Frame::window(ui.style())
                .inner_margin(egui::style::Margin::symmetric(
                    (grid_width - rect_side * self.dimensions.x as f32) / 2.0,
                    (grid_height - rect_side * self.dimensions.y as f32) / 2.0,
                ))
                .show(ui, |ui| {
                    egui::Grid::new("board").spacing([0., 0.]).show(ui, |ui| {
                        for i in 0..self.dimensions.x {
                            for j in 0..self.dimensions.y {
                                let btn_txt = self
                                    .curr_sol
                                    .and_then(|idx| {
                                        (&self.solutions[idx])
                                            .iter()
                                            .position(|&pos| pos.x == i && pos.y == j)
                                            .map(|step_idx| (step_idx + 1).to_string())
                                    })
                                    .unwrap_or_default();
                                let btn = egui::Button::new(btn_txt)
                                    .min_size(button_size)
                                    .rounding(egui::Rounding::ZERO)
                                    .fill(if (i + j) % 2 == 0 {
                                        light_color
                                    } else {
                                        dark_color
                                    });
                                let square = ui.add(btn);
                                if square.clicked() {
                                    self.curr_sol = None;
                                    self.solutions.truncate(0);
                                    let (tx, rx) = mpsc::channel();
                                    self.receiver = Some(rx);
                                    solve(self.dimensions, Pos { x: i, y: j }, tx);
                                };
                            }
                            ui.end_row();
                        }
                    });
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Knight's Tour",
        NativeOptions::default(),
        Box::new(|_| Box::<App>::default()),
    )
}

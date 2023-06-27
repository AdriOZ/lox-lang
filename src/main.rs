#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ast;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

use eframe::egui;

pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 800.0)),
        ..Default::default()
    };

    let mut source_code = "".to_owned();
    let mut tokens = "".to_owned();
    let mut tree = "".to_owned();

    eframe::run_simple_native("Lox Language", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    let mut sc = scanner::Scanner::new(&source_code.clone());
                    let tk = sc.parse();
                    tokens = format!("{:#?}", tk);
                    let mut tr = parser::Parser::new(tk);
                    let ep = tr.parse();
                    tree = format!("{:#?}", ep);

                    ui.heading("Editor");
                    ui.add_sized(
                        [ui.available_width(), 220.0],
                        egui::TextEdit::multiline(&mut source_code),
                    );

                    ui.heading("Tokens");
                    ui.add_sized(
                        [ui.available_width(), 220.0],
                        egui::TextEdit::multiline(&mut tokens),
                    );

                    ui.heading("Abstract Syntax Tree");
                    ui.add_sized(
                        [ui.available_width(), 220.0],
                        egui::TextEdit::multiline(&mut tree),
                    );
                });
            });
        });
    })
}

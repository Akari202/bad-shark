#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;

use bad_shark::BSApp;
use eframe::egui;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Bad Shark",
        options,
        Box::new(|cc| Ok(Box::new(BSApp::new(cc))))
    )?;
    Ok(())
}

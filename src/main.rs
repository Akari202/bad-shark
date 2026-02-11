#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;

use bad_shark::BSApp;
use eframe::egui;
use env_logger::Env;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // egui_logger::builder().init().unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Bad Shark",
        options,
        Box::new(|cc| {
            let app = BSApp::new(cc).map_err(|e| e.to_string())?;
            Ok(Box::new(app))
        })
    )?;
    Ok(())
}

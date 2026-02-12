#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
use std::io::Write;

use bad_shark::BSApp;
use eframe::egui;
use env_logger::{Builder, Env};

fn main() -> Result<(), Box<dyn Error>> {
    // egui_logger::builder().init().unwrap();
    let crate_name = env!("CARGO_PKG_NAME").replace("-", "_");
    let filter = format!("info,{}=debug", crate_name);
    Builder::from_env(Env::default().default_filter_or(filter))
        .format(|buf, record| {
            let style = buf.default_level_style(record.level()).bold();

            writeln!(
                buf,
                "[{}{:>5}{} {}] {}",
                style.render(),
                record.level(),
                style.render_reset(),
                record.target(),
                record.args()
            )
        })
        .init();

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

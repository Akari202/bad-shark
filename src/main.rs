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
// use bad_shark::run;
//
// fn main() {
//     // println!("Front Motion Ratios: {:?}", test_front.motion_ratios());
//     // println!("Front Caster Angle: {:.3}", test_front.caster_angle().to_degrees());
//     // println!("Front Upright Ball Joint Distance: {:.3}in", test_front.outer_upright_mounting_distance() / 25.4);
//     // dbg!(test_front.rotate_upper_aarm());
//
//     pollster::block_on(run());
// }

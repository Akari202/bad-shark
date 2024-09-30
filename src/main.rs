use crate::test_car::TEST_CAR;

mod points;
mod test_car;
mod units;

// NOTE: Internally everything is in millimeters and radians, i want to move way from units.rs

fn main() {
    println!("Wheelbase: {:.3}", TEST_CAR.wheelbase());
    println!("Caster angle: {:.3}", TEST_CAR.front.caster());
    println!("Front and rear track width: {:.0}, {:.0}", TEST_CAR.track_width().0, TEST_CAR.track_width().1);
    println!("Front camber angle: {:.3}", TEST_CAR.front.wheel.camber());
    println!("Rear camber angle: {:.3}", TEST_CAR.rear.wheel.camber());
    println!("Front shock compression: {:.3}", TEST_CAR.front.damper.compression_distance());
    println!("Rear shock compression: {:.3}", TEST_CAR.rear.damper.compression_distance());
    println!("Front upper wishbone angle at full droop: {:.3}", TEST_CAR.front.upper_wishbone_angle_from_damper_extension(0.8).expect("Error"));
}



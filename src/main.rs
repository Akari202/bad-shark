use crate::test_car::TEST_CAR;

mod points;
mod test_car;


fn main() {
    println!("Wheelbase: {:.3} mm", TEST_CAR.wheelbase());
    println!("Caster angle: {:.3} degrees", radians_to_degrees(TEST_CAR.front.caster()));
    println!("Front track width: {:.3} mm", TEST_CAR.front.track_width());
    println!("Rear track width: {:.3} mm", TEST_CAR.rear.track_width());
    println!("Front camber angle: {:.3} degrees", radians_to_degrees(TEST_CAR.front.wheel.camber()));
    println!("Rear camber angle: {:.3} degrees", radians_to_degrees(TEST_CAR.rear.wheel.camber()));
}

fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / std::f64::consts::PI
}

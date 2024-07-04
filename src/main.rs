use crate::test_car::TEST_CAR;

mod points;
mod test_car;
mod units;


fn main() {
    println!("Wheelbase: {}", TEST_CAR.wheelbase());
    println!("Caster angle: {}", TEST_CAR.front.caster());
    println!("Front and rear track width: {}, {}", TEST_CAR.track_width().0, TEST_CAR.track_width().1);
    println!("Front camber angle: {}", TEST_CAR.front.wheel.camber());
    println!("Rear camber angle: {}", TEST_CAR.rear.wheel.camber());
    println!("Front shock compression: {}", TEST_CAR.front.damper.compression_distance());
    println!("Rear shock compression: {}", TEST_CAR.rear.damper.compression_distance());
}



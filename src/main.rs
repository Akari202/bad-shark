use crate::test_car::get_test_front;

mod units;
mod car;
mod test_car;

pub const ANGLE_EPSILON_DEGREES: f64 = 5.0;

fn main() {
    let test_front = get_test_front();

    // println!("Front Motion Ratios: {:?}", test_front.motion_ratios());
    // println!("Front Caster Angle: {:.3}", test_front.caster_angle().to_degrees());
    // println!("Front Upright Ball Joint Distance: {:.3}in", test_front.outer_upright_mounting_distance() / 25.4);
    dbg!(test_front.rotate_upper_aarm());
}



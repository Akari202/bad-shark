use bad_shark::run;
use bad_shark::test_car::get_test_front;


fn main() {
    let test_front = get_test_front();

    // println!("Front Motion Ratios: {:?}", test_front.motion_ratios());
    // println!("Front Caster Angle: {:.3}", test_front.caster_angle().to_degrees());
    // println!("Front Upright Ball Joint Distance: {:.3}in", test_front.outer_upright_mounting_distance() / 25.4);
    // dbg!(test_front.rotate_upper_aarm());

    pollster::block_on(run());
}



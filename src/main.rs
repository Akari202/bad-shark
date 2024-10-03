use bad_shark::run;

fn main() {
    // println!("Front Motion Ratios: {:?}", test_front.motion_ratios());
    // println!("Front Caster Angle: {:.3}", test_front.caster_angle().to_degrees());
    // println!("Front Upright Ball Joint Distance: {:.3}in", test_front.outer_upright_mounting_distance() / 25.4);
    // dbg!(test_front.rotate_upper_aarm());

    pollster::block_on(run());
}



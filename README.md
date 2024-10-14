# Bad SHARK

An experimental suspension kinematics program intended to (eventually) provide similar features and calculations as Lotus SHARK or SusProg. 

This is a side project, work will be sporadic.


This was originally a command line based program, but it made debugging a pain,
in order to properly visualize what was happening I was printing sets of points and importing into solidworks.
So i broke down and hacked some wgpu together to be able to properly see what's happening.
Tbh im kinda suprised how smoothly the whole thing went, it only really took an evening to get a window
with geometry in it, couldn't be vulkan. Graphics and UI are a secondary priority, right now im focusing on
getting complete calculations for the double wishbone and inverted h-arm with camber link style suspensions
as is employed on the Clarkson Baja Team's car.

## Usage

If you want to use I don't particularly want to share my car's geometry. 
The geometry for now is stored in `src/car/test_car.rs` which is included below with all values turned to 0. 

```rust
use vec_utils::vec3d::Vec3d;
use crate::car::Car;
use crate::car::front::Front;
use crate::car::members::a_arm::AArm;
use crate::car::members::h_arm::HArm;
use crate::car::members::link::Link;
use crate::car::rear::Rear;

pub(crate) fn get_test_car() -> Car {
    let translation = 0.0;
    Car {
        front: get_test_front(translation),
        rear: get_test_rear(translation)
    }
}

fn get_test_front(translation: f64) -> Front {
    let mut front = Front {
        upper_datum: Vec3d::new(0.0 - translation, 0.0, 0.0),
        upper: AArm::from_global(
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Some(Vec3d::new(0.0 - translation, 0.0, 0.0))
        ),
        lower_datum: Vec3d::new(0.0 - translation, 0.0, 0.0),
        lower: AArm::from_global(
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            None
        ),
        damper_body: Vec3d::new(0.0 - translation, 0.0, 0.0),
        upright_distance: 0.0
    };
    front.upright_distance = front.outer_upright_mounting_distance();
    front
}

fn get_test_rear(translation: f64) -> Rear {
    let mut rear = Rear {
        harm_datum: Vec3d::new(0.0 - translation, 0.0, 0.0),
        harm: HArm::from_global(
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0)
        ),
        camber_link_datum: Vec3d::new(0.0- translation, 0.0, 0.0),
        camber_link: Link::from_global(
            Vec3d::new(0.0 - translation, 0.0, 0.0),
            Vec3d::new(0.0 - translation, 0.0, 0.0)
        ),
        damper_body: Vec3d::new(0.0 - translation, 0.0, 0.0),
        upright_distance: 0.0,
    };
    rear
}
```
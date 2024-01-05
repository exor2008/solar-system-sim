use nalgebra::Vector3;
use solar_system_sim::{constants::*, Body};

fn main() {
    let mut sun = Body::new(
        0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        SUN_MASS,
    );

    let mut earth = Body::new(
        1,
        Vector3::new(-1.0 * AU, 0.0, 0.0),
        Vector3::new(0.0, EARTH_VEL, 0.0),
        EARTH_MASS,
    );

    loop {
        for i in 0..2 {
            let mut others = vec![&mut sun, &mut earth];
            let body: &mut Body = others.remove(i);
            body.update(others);
        }
        println!("{}", earth.scaled_coord());
    }
}

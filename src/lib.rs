pub mod constants;

use constants::*;
use nalgebra::Vector3;
pub struct Body {
    pub id: u32,
    pub coord: Vector3<f64>,
    pub vel: Vector3<f64>,
    pub mass: f64,
}

impl Body {
    pub fn new(id: u32, coord: Vector3<f64>, vel: Vector3<f64>, mass: f64) -> Self {
        Body {
            id,
            coord,
            vel,
            mass,
        }
    }

    pub fn attraction(&self, other: &Body) -> Vector3<f64> {
        let diff = other.coord - self.coord;
        let distance = diff.norm();

        let phi = diff.y.atan2(diff.x);
        let theta = (diff.z / distance).acos();

        let force = (G * self.mass * other.mass) / (distance * distance);

        let x = force * theta.sin() * phi.cos();
        let y = force * theta.sin() * phi.sin();
        let z = force * theta.cos();

        Vector3::new(x, y, z)
    }

    pub fn update(&mut self, bodies: Vec<&mut Body>) {
        let mut force: Vector3<f64> = Vector3::default();

        for other in bodies {
            if self.id == other.id {
                continue;
            }

            force += self.attraction(other);
        }

        self.vel += force / self.mass * TIMESTEP;
        self.coord += self.vel * TIMESTEP;
    }

    pub fn scaled_coord(&self) -> Vector3<f64> {
        self.coord * SCALE
    }
}

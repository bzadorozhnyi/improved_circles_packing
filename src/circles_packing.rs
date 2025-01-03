use nalgebra::DVector;

use crate::{circle::Circle, point::Point, utils::FloatType};

pub struct CirclesPacking {
    pub main_circle: Circle,
    pub inner_circles: Vec<Circle>,
}

impl CirclesPacking {
    pub fn new(main_circle: Circle, inner_circles: Vec<Circle>) -> Self {
        Self {
            main_circle,
            inner_circles,
        }
    }

    pub fn from_coords_vec_and_radiuses(x: DVector<FloatType>, radiuses: &Vec<FloatType>) -> Self {
        let main_circle = Circle::new(x[x.len() - 1], Point::default());

        let inner_circles = (0..x.len() / 2)
            .into_iter()
            .map(|i| {
                Circle::new(
                    radiuses[i],
                    Point {
                        x: x[i],
                        y: x[i + x.len() / 2],
                    },
                )
            })
            .collect();

        Self {
            main_circle,
            inner_circles,
        }
    }

    // TODO: replace panic with result
    pub fn into_coords_vec(&self) -> DVector<FloatType> {
        let data: Vec<FloatType> = ([
            Vec::from_iter(
                self.inner_circles
                    .iter()
                    .map(|c| c.center.expect("Valid center").x),
            ),
            Vec::from_iter(
                self.inner_circles
                    .iter()
                    .map(|c| c.center.expect("Valid center").y),
            ),
            vec![self.main_circle.radius],
        ])
        .concat();

        DVector::from_vec(data)
    }

    pub fn is_valid_pack(&self) -> bool {
        if self
            .inner_circles
            .iter()
            .any(|circle| !circle.is_inside_main_circle(self.main_circle.radius))
        {
            return false;
        }

        for i in 0..self.inner_circles.len() {
            for j in i + 1..self.inner_circles.len() {
                if self.inner_circles[i].overlap(&self.inner_circles[j]) {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        println!("{}", self.main_circle.radius);
        println!("{}", self.is_valid_pack());
        for circle in &self.inner_circles {
            println!(
                "{} {:.15} {:.15}",
                circle.radius,
                circle.center.unwrap().x,
                circle.center.unwrap().y
            );
        }
        println!();
    }
}

pub mod builder;

use rand::rngs::StdRng;
use rand::Rng;

use crate::circle::*;
use crate::circles_packing::CirclesPacking;
use crate::point::Point;
use crate::utils::FloatType;

pub use self::builder::HeuristicAlgorithmBuilder;

pub struct HeuristicAlgorithm {
    iterations: usize,
    rng: StdRng,
    radiuses: Vec<FloatType>,
    max_small_circle_radius: FloatType,
    delta: FloatType,
}

impl HeuristicAlgorithm {
    pub fn find(&mut self, skip_iteration: Option<usize>) -> CirclesPacking {
        if let Some(skip_iteration) = skip_iteration {
            for _ in 0..skip_iteration {
                self.swap_circles();
            }
        }

        let radiuses_sum = (self.radiuses.iter().sum::<FloatType>() as FloatType).ceil();

        let mut main_circle_radius: FloatType = radiuses_sum;

        let mut new_innner_circles_pack: Vec<Circle> = Vec::new();
        let mut inner_circles_pack: Vec<Circle> = (0..self.radiuses.len())
            .map(|_| Circle::default())
            .collect();

        for iter in 0..self.iterations {
            if iter % 10_000 == 0 {
                println!("iter = {iter}");
            }

            let (mut left, mut right) = (0 as FloatType, main_circle_radius);

            while right - left >= 1e-4 {
                // let middle = (left + right) / 2.0;
                let middle = left + (right - left) / 1.02;
                // let middle = right - (right - left) / 500.0;

                if let Some(circles) = self.pack_circles(middle) {
                    right = middle;
                    new_innner_circles_pack = circles;
                } else {
                    left = middle;
                }
            }

            let new_main_circle_radius = right;
            if new_main_circle_radius < main_circle_radius
                && is_valid_pack(new_main_circle_radius, &new_innner_circles_pack)
            {
                main_circle_radius = new_main_circle_radius;
                inner_circles_pack = new_innner_circles_pack.clone();
            }

            self.swap_circles();
        }

        CirclesPacking::new(
            Circle::new(main_circle_radius, Point::default()),
            inner_circles_pack,
        )
    }

    fn pack_first_layer(
        &self,
        circles: &mut [Circle],
        placed_circle_indexes: &mut Vec<usize>,
        main_circle_radius: FloatType,
    ) {
        let get_rotated_point = |y_coord: FloatType, angle: FloatType| -> Point {
            Point {
                x: y_coord * angle.sin(),
                y: y_coord * angle.cos(),
            }
        };

        let angle_between_prev_next_circles = |prev_radius: FloatType,
                                               next_radius: FloatType,
                                               main_circle_radius: FloatType|
         -> FloatType {
            let (a, b, c) = (
                main_circle_radius - prev_radius - self.delta,
                prev_radius + next_radius + self.delta,
                main_circle_radius - next_radius - self.delta,
            );

            ((a.powi(2) + c.powi(2) - b.powi(2)) / (2.0 * a * c)).acos()
        };

        circles[0].center = Some(Point {
            x: 0.0,
            y: main_circle_radius - self.delta - circles[0].radius,
        });

        placed_circle_indexes.push(0);
        let mut prev_circle_angle: FloatType = 0.0;

        for index in 1..circles.len() {
            if (1.0..=self.max_small_circle_radius).contains(&circles[index].radius) {
                continue;
            }

            let next_circle_angle = prev_circle_angle
                + angle_between_prev_next_circles(
                    circles[placed_circle_indexes[placed_circle_indexes.len() - 1]].radius,
                    circles[index].radius,
                    main_circle_radius,
                );

            let new_circle = Circle {
                center: Some(get_rotated_point(
                    main_circle_radius - self.delta - circles[index].radius,
                    next_circle_angle,
                )),
                radius: circles[index].radius,
            };

            if !new_circle.is_overlap_quad(circles) {
                circles[index] = new_circle;
                placed_circle_indexes.push(index);

                prev_circle_angle = next_circle_angle;
            }
        }
    }

    /// Find tangent circle center to previous and main circles.
    fn find_tangent_circle_center(
        &self,
        prev_circle: &Circle,
        next_circle: &Circle,
        main_circle_radius: FloatType,
    ) -> Option<Vec<Point>> {
        if let Some(prev_circle_center) = prev_circle.center {
            let (xp, yp, rp) = (
                prev_circle_center.x,
                prev_circle_center.y,
                prev_circle.radius,
            );
            let rn = next_circle.radius;

            let t = (xp.powi(2) + yp.powi(2)) - (rp + rn + self.delta).powi(2)
                + (main_circle_radius - self.delta - rn).powi(2);
            let p = 2.0 * yp;

            if yp == 0.0 {
                if xp == 0.0 {
                    return None;
                }

                let xn = (xp.powi(2) - (rp + rn + self.delta).powi(2)
                    + (main_circle_radius - self.delta - rn).powi(2))
                    / (2.0 * xp);

                let yn = ((rp + rn + self.delta).powi(2) - (xp - xn).powi(2)).sqrt();

                return Some(vec![Point { x: xn, y: yn }]);
            }

            let a = p.powi(2) + 4.0 * xp.powi(2);
            let b = -4.0 * xp * t;
            let c = t.powi(2) - p.powi(2) * (main_circle_radius - self.delta - rn).powi(2);

            let d = b.powi(2) - 4.0 * a * c;

            if d < 0.0 {
                return None;
            }

            let sqrt_d = d.sqrt();

            let xn_1 = (-b - sqrt_d) / (2.0 * a);
            let xn_2 = (-b + sqrt_d) / (2.0 * a);

            let yn_1 = (t - 2.0 * xp * xn_1) / p;
            let yn_2 = (t - 2.0 * xp * xn_2) / p;

            Some(vec![Point { x: xn_1, y: yn_1 }, Point { x: xn_2, y: yn_2 }])
        } else {
            None
        }
    }

    fn pack_small_circle_layer(
        &self,
        circles: &mut [Circle],
        placed_circle_indexes: &Vec<usize>,
        main_circle_radius: FloatType,
    ) {
        for placed_circle_index in placed_circle_indexes {
            'circles_loop: for i in 0..circles.len() {
                if circles[i].center.is_some()
                    || !(1.0..=self.max_small_circle_radius).contains(&circles[i].radius)
                {
                    continue;
                }

                if let Some(points) = self.find_tangent_circle_center(
                    &circles[*placed_circle_index],
                    &circles[i],
                    main_circle_radius,
                ) {
                    for point in points {
                        let new_circle: Circle = Circle {
                            radius: circles[i].radius,
                            center: Some(point),
                        };

                        if new_circle.is_inside_main_circle_quad(main_circle_radius)
                            && !new_circle.is_overlap_quad(circles)
                        {
                            circles[i] = new_circle;
                            break 'circles_loop;
                        }
                    }
                }
            }
        }
    }

    /// Find center of third circle, closest to (0, 0).
    fn find_third_circle_center(&self, c1: &Circle, c2: &Circle, r3: FloatType) -> Option<Point> {
        if let (Some(c1_center), Some(c2_center)) = (c1.center, c2.center) {
            if (c1_center.x - c2_center.x).powi(2) + (c1_center.y - c2_center.y).powi(2)
                > (c1.radius + 2.0 * r3 + c2.radius).powi(2)
            {
                return None;
            }

            let (x1, y1, r1) = (c1_center.x, c1_center.y, c1.radius);
            let (x2, y2, r2) = (c2_center.x, c2_center.y, c2.radius);

            if (y1 - y2).abs() == 0.0 {
                if (x1 - x2).abs() == 0.0 {
                    return None;
                }

                let x_3 = ((x1.powi(2) - x2.powi(2)) - (r1 + r3).powi(2) + (r2 + r3).powi(2))
                    / (2.0 * (x1 - x2));

                let y_3 = ((x1 - x_3).powi(2) - (r1 + r3).powi(2)).sqrt() + y1;

                return Some(Point { x: x_3, y: y_3 });
            }

            let t =
                x1.powi(2) - x2.powi(2) + y1.powi(2) - y2.powi(2) - (r1 + r3 + self.delta).powi(2)
                    + (r2 + r3 + self.delta).powi(2);
            let p = 2.0 * (y1 - y2);

            let a = p.powi(2) + 4.0 * (x1 - x2).powi(2);

            let b = -2.0 * x1 * p.powi(2) + 4.0 * (y1 * p - t) * (x1 - x2);
            let c = p.powi(2) * (x1.powi(2) + y1.powi(2) - (r1 + r3 + self.delta).powi(2))
                - 2.0 * y1 * p * t
                + t.powi(2);

            let d = b.powi(2) - 4.0 * a * c;

            if d < 0.0 {
                return None;
            }

            let sqrt_d = d.sqrt();
            let x3_1 = (-b - sqrt_d) / (2.0 * a);
            let x3_2 = (-b + sqrt_d) / (2.0 * a);

            let y3_1 = (t - 2.0 * (x1 - x2) * x3_1) / p;
            let y3_2 = (t - 2.0 * (x1 - x2) * x3_2) / p;

            let c3_center = if x3_1.powi(2) + y3_1.powi(2) < x3_2.powi(2) + y3_2.powi(2) {
                Point { x: x3_1, y: y3_1 }
            } else {
                Point { x: x3_2, y: y3_2 }
            };

            Some(c3_center)
        } else {
            None
        }
    }

    fn eval_alpha(&self, c1: &Circle, c2: &Circle, r3: FloatType) -> FloatType {
        let a = c1.radius + r3 + self.delta;
        let b = c2.radius + r3 + self.delta;
        let c = c1.distance(c2) + self.delta;

        (a * a + b * b - c * c) / (2.0 * a * b)
    }

    fn pack_other_layers(
        &self,
        circles: &mut [Circle],
        mut placed_circle_indexes: Vec<usize>,
        main_circle_radius: FloatType,
    ) {
        let cycle_index =
            |vector: &Vec<usize>, index: usize| -> usize { vector[index % vector.len()] };

        while !placed_circle_indexes.is_empty() {
            let mut new_placed_circle_indexes: Vec<usize> = Vec::new();

            for placed_circle_index in 0..placed_circle_indexes.len() {
                let (mut candidate_alpha, mut candidate_circle_index, mut candidate_circle) =
                    (FloatType::NEG_INFINITY, 0, Circle::default());
                for i in 0..circles.len() {
                    if circles[i].center.is_some() {
                        continue;
                    }

                    for shift in 1..=2.min(placed_circle_indexes.len()) {
                        let first_index = placed_circle_indexes[placed_circle_index];
                        let second_index =
                            cycle_index(&placed_circle_indexes, placed_circle_index + shift);

                        let alpha = self.eval_alpha(
                            &circles[first_index],
                            &circles[second_index],
                            circles[i].radius,
                        );

                        if alpha < candidate_alpha {
                            continue;
                        }

                        let new_circle_center: Option<Point> = self.find_third_circle_center(
                            &circles[first_index],
                            &circles[second_index],
                            circles[i].radius,
                        );

                        if new_circle_center.is_none() {
                            continue;
                        }

                        let new_circle: Circle = Circle {
                            center: new_circle_center,
                            radius: circles[i].radius,
                        };

                        if new_circle.is_inside_main_circle_quad(main_circle_radius)
                            && !new_circle.is_overlap_quad(circles)
                            && alpha > candidate_alpha
                        {
                            (candidate_alpha, candidate_circle_index, candidate_circle) =
                                (alpha, i, new_circle);
                        }
                    }
                }

                if candidate_alpha != FloatType::NEG_INFINITY {
                    circles[candidate_circle_index] = candidate_circle;
                    new_placed_circle_indexes.push(candidate_circle_index);
                }
            }
            placed_circle_indexes = new_placed_circle_indexes;
        }
    }

    fn pack_circles(&self, main_circle_radius: FloatType) -> Option<Vec<Circle>> {
        let mut circles: Vec<Circle> = self
            .radiuses
            .iter()
            .map(|&radius| Circle::with_radius(radius))
            .collect();

        let mut placed_circle_indexes: Vec<usize> = Vec::new();

        self.pack_first_layer(&mut circles, &mut placed_circle_indexes, main_circle_radius);

        self.pack_small_circle_layer(&mut circles, &placed_circle_indexes, main_circle_radius);

        self.pack_other_layers(&mut circles, placed_circle_indexes, main_circle_radius);

        if is_valid_pack(main_circle_radius, &circles) {
            Some(circles)
        } else {
            None
        }
    }

    fn swap_circles(&mut self) {
        let circles_number = self.radiuses.len();
        self.radiuses.swap(
            self.rng.gen_range(0..circles_number),
            self.rng.gen_range(0..circles_number),
        );
    }
}

pub fn is_valid_pack(main_circle_radius: FloatType, circles: &[Circle]) -> bool {
    if circles
        .iter()
        .any(|circle| !circle.is_inside_main_circle(main_circle_radius))
    {
        return false;
    }

    for i in 0..circles.len() {
        for j in i + 1..circles.len() {
            if circles[i].overlap(&circles[j]) {
                return false;
            }
        }
    }

    true
}

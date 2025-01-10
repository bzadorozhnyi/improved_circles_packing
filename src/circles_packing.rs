use nalgebra::DVector;
use plotters::{
    chart::ChartBuilder,
    drawing::IntoDrawingArea,
    prelude::{BitMapBackend, EmptyElement},
    style::{full_palette::GREEN_800, Color, ShapeStyle, TextStyle, WHITE},
};

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

    pub fn plot(&self, path: &str) {
        let img_size = 1000;
        let plot_size = self.main_circle.radius * 1.1;
        let label_area_size = 85;

        let root = BitMapBackend::new(path, (img_size, img_size)).into_drawing_area();
        root.fill(&WHITE).ok();

        let mut chart = ChartBuilder::on(&root)
            .set_all_label_area_size(label_area_size)
            .build_cartesian_2d(-plot_size..plot_size, -plot_size..plot_size)
            .unwrap();

        chart
            .configure_mesh()
            .disable_mesh()
            .x_labels(10)
            .y_labels(10)
            .label_style(TextStyle::from(("bebas neue", 30)))
            .draw()
            .ok();

        let root = chart.plotting_area();

        let convert_radius = |radius: FloatType| {
            return (radius * (img_size - 2 * label_area_size) as FloatType) / (2.0 * plot_size);
        };

        let set_circle = |c: &Circle| {
            return EmptyElement::at((c.center.unwrap().x, c.center.unwrap().y))
                + plotters::element::Circle::new(
                    (0, 0),
                    3,
                    ShapeStyle {
                        color: GREEN_800.mix(0.8),
                        filled: true,
                        stroke_width: 2,
                    },
                )
                + plotters::element::Circle::new(
                    (0, 0),
                    convert_radius(c.radius),
                    ShapeStyle {
                        color: GREEN_800.mix(0.8),
                        filled: false,
                        stroke_width: 2,
                    },
                );
        };

        let main_circle = Circle::new(self.main_circle.radius, Point { x: 0.0, y: 0.0 });
        root.draw(&set_circle(&main_circle)).ok();
        self.inner_circles.iter().for_each(|c| {
            root.draw(&set_circle(&c)).ok();
        });

        root.present().ok();
    }
}

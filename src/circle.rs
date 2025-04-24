use crate::{point::Point, utils::FloatType};

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    pub center: Option<Point>,
    pub radius: FloatType,
}

impl Circle {
    pub fn new(radius: FloatType, center: Point) -> Self {
        Circle {
            center: Some(center),
            radius,
        }
    }

    pub fn with_radius(radius: FloatType) -> Self {
        Circle {
            center: None,
            radius,
        }
    }

    pub fn overlap(&self, other: &Circle) -> bool {
        if let (Some(center_self), Some(center_other)) = (self.center, other.center) {
            let distance = ((center_self.x - center_other.x).powi(2)
                + (center_self.y - center_other.y).powi(2))
            .sqrt();
            let radius_sum = self.radius + other.radius;

            distance <= radius_sum
        } else {
            false
        }
    }

    pub fn overlap_quad(&self, other: &Circle) -> bool {
        if let (Some(center_self), Some(center_other)) = (self.center, other.center) {
            let distance =
                (center_self.x - center_other.x).powi(2) + (center_self.y - center_other.y).powi(2);
            let radius_sum = (self.radius + other.radius).powi(2);

            distance <= radius_sum
        } else {
            false
        }
    }

    pub fn is_overlap(&self, circles: &[Circle]) -> bool {
        circles.iter().any(|c| self.overlap(c))
    }

    pub fn is_overlap_quad(&self, circles: &[Circle]) -> bool {
        circles.iter().any(|c| self.overlap_quad(c))
    }

    pub fn distance(&self, other: &Circle) -> FloatType {
        if let (Some(center_self), Some(center_other)) = (self.center, other.center) {
            let distance = ((center_self.x - center_other.x).powi(2)
                + (center_self.y - center_other.y).powi(2))
            .sqrt();

            distance
        } else {
            FloatType::INFINITY
        }
    }

    pub fn inside(&self, other: &Circle) -> bool {
        if let (Some(center_self), Some(center_other)) = (self.center, other.center) {
            let distance = ((center_self.x - center_other.x).powi(2)
                + (center_self.y - center_other.y).powi(2))
            .sqrt();

            distance <= (other.radius - self.radius)
        } else {
            false
        }
    }

    pub fn is_inside_main_circle(&self, main_circle_radius: FloatType) -> bool {
        if let Some(center) = self.center {
            let distance = (center.x.powi(2) + center.y.powi(2)).sqrt();
            distance <= (main_circle_radius - self.radius)
        } else {
            false
        }
    }

    pub fn is_inside_main_circle_quad(&self, main_circle_radius: FloatType) -> bool {
        if let Some(center) = self.center {
            center.x.powi(2) + center.y.powi(2) <= (main_circle_radius - self.radius).powi(2)
        } else {
            false
        }
    }
}

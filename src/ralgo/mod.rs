pub mod builder;

use nalgebra::{DMatrix, DVector};

use crate::utils::FloatType;

pub use self::builder::RAlgorithmBuilder;

pub struct RAlgorithm {
    alpha: FloatType,
    q1: FloatType,
    epsx: FloatType,
    epsg: FloatType,
    max_iterations: usize,
    calcfg: Box<dyn Fn(&DVector<FloatType>) -> (FloatType, DVector<FloatType>)>,
}

impl RAlgorithm {
    pub fn evaluate(&self, mut x: DVector<FloatType>, mut h: FloatType) -> DVector<FloatType> {
        let mut b_matrix = DMatrix::<FloatType>::identity(x.len(), x.len());

        let mut result_x = x.clone();
        let (mut result_f, mut g0) = (self.calcfg)(&result_x);
        let beta_v = 1.0 / self.alpha - 1.0;

        if g0.norm() < self.epsg {
            return result_x;
        }

        for _ in 0..self.max_iterations {
            let mut g1: DVector<FloatType> = b_matrix.tr_mul(&g0);

            let dx = &b_matrix * (&g1 / g1.norm());
            let dx_norm = dx.norm();

            let mut f;
            let (mut d, mut ls, mut ddx) = (1.0 as FloatType, 0_u32, 0.0 as FloatType);
            while d > 0.0 {
                x.axpy(-h, &dx, 1.0);
                ddx += h * dx_norm;

                (f, g1) = (self.calcfg)(&x);
                if f < result_f {
                    (result_f, result_x) = (f, x.clone());
                }

                if g1.norm() < self.epsg {
                    return result_x;
                }

                ls += 1;
                if ls % 3 == 0 {
                    h *= 1.1;
                }

                if ls > 500 {
                    return result_x;
                }

                d = dx.dot(&g1);
            }

            if ls == 1 {
                h *= self.q1;
            }

            if ddx < self.epsx {
                return result_x;
            }

            let r = b_matrix.tr_mul(&(&g1 - &g0)).normalize();
            b_matrix += beta_v * (&b_matrix * &r) * &r.transpose();
            g0 = g1;
        }

        result_x
    }
}

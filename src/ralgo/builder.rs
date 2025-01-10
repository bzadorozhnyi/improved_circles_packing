use nalgebra::DVector;

use crate::utils::FloatType;

use super::RAlgorithm;

type CalcfgType = dyn Fn(&DVector<FloatType>) -> (FloatType, DVector<FloatType>);

pub struct RAlgorithmBuilder {
    alpha: FloatType,
    q1: FloatType,
    epsx: FloatType,
    epsg: FloatType,
    max_iterations: usize,
    calcfg: Box<CalcfgType>,
}

impl Default for RAlgorithmBuilder {
    fn default() -> Self {
        Self {
            alpha: 1.5 as FloatType,
            q1: 1.0,
            epsx: 1e-6,
            epsg: 1e-7,
            max_iterations: 3_000,
            calcfg: Box::new(|_x: &DVector<FloatType>| {
                panic!("Daefault calcfg should be overriden before use!")
            }),
        }
    }
}

impl RAlgorithmBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alpha(mut self, alpha: FloatType) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn q1(mut self, q1: FloatType) -> Self {
        self.q1 = q1;
        self
    }

    pub fn epsx(mut self, epsx: FloatType) -> Self {
        self.epsx = epsx;
        self
    }

    pub fn epsg(mut self, epsg: FloatType) -> Self {
        self.epsg = epsg;
        self
    }

    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn calcfg(mut self, calcfg: Box<CalcfgType>) -> Self {
        self.calcfg = calcfg;
        self
    }

    pub fn build(self) -> RAlgorithm {
        RAlgorithm {
            alpha: self.alpha,
            q1: self.q1,
            epsx: self.epsx,
            epsg: self.epsg,
            max_iterations: self.max_iterations,
            calcfg: self.calcfg,
        }
    }
}

use nalgebra::DVector;

use crate::{ralgo::RAlgorithm, utils::FloatType};

pub fn dichotomy_step_ralgo(
    mut x: DVector<FloatType>,
    reset_step: bool,
    eps: FloatType,
    ralgo: RAlgorithm,
) -> DVector<FloatType> {
    let mut step_size = 40.96;

    let last = |x: &DVector<FloatType>| -> FloatType { x[x.len() - 1] };

    while step_size >= 0.01 {
        println!("step = {step_size}");
        let y = ralgo.evaluate(x.clone(), step_size);

        if (last(&x) - last(&y)) / last(&x) > eps {
            x = y;
            if reset_step {
                step_size = 40.96;
            }
        } else {
            step_size /= 2.0;
        }
    }

    x
}

pub fn smart_dichotomy_step_ralgo(
    mut x: DVector<FloatType>,
    eps: FloatType,
    ralgo: RAlgorithm,
) -> DVector<FloatType> {
    let mut step_size = 40.96;

    let last = |x: &DVector<FloatType>| -> FloatType { x[x.len() - 1] };

    while step_size >= 0.01 {
        println!("step = {step_size}");
        let y = ralgo.evaluate(x.clone(), step_size);

        if (last(&x) - last(&y)) / last(&x) > eps {
            x = y;
            step_size = (step_size * 2.0).min(40.96);
        } else {
            step_size /= 2.0;
        }
    }

    x
}

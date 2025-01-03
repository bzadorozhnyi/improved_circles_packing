use nalgebra::DVector;

use crate::{
    calcfg::calcfg, circles_packing::CirclesPacking, ralgo::builder::RAlgorithmBuilder,
    utils::FloatType,
};

pub fn dichotomy_step_ralgo(
    circle_packing: CirclesPacking,
    reset_step: bool,
    eps: FloatType,
    ralgo_builder: RAlgorithmBuilder,
) -> CirclesPacking {
    let inner_circles_radiuses = circle_packing
        .inner_circles
        .iter()
        .map(|c| c.radius)
        .collect::<Vec<_>>();

    let inner_clone = inner_circles_radiuses.clone();
    let mut x: DVector<FloatType> = circle_packing.into_coords_vec();

    let mut step_size = 40.96;

    let last = |x: &DVector<FloatType>| -> FloatType { x[x.len() - 1] };

    let calcfg = move |x: &DVector<FloatType>| -> (FloatType, DVector<FloatType>) {
        calcfg(&x, &inner_circles_radiuses)
    };

    let ralgo = ralgo_builder.calcfg(Box::new(calcfg)).build();

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

    CirclesPacking::from_coords_vec_and_radiuses(x, &inner_clone)
}

pub fn smart_dichotomy_step_ralgo(
    circle_packing: CirclesPacking,
    eps: FloatType,
    ralgo_builder: RAlgorithmBuilder,
) -> CirclesPacking {
    let inner_circles_radiuses = circle_packing
        .inner_circles
        .iter()
        .map(|c| c.radius)
        .collect::<Vec<_>>();

    let inner_clone = inner_circles_radiuses.clone();
    let mut x: DVector<FloatType> = circle_packing.into_coords_vec();

    let mut step_size = 40.96;

    let last = |x: &DVector<FloatType>| -> FloatType { x[x.len() - 1] };

    let calcfg = move |x: &DVector<FloatType>| -> (FloatType, DVector<FloatType>) {
        calcfg(&x, &inner_circles_radiuses)
    };

    let ralgo = ralgo_builder.calcfg(Box::new(calcfg)).build();

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

    CirclesPacking::from_coords_vec_and_radiuses(x, &inner_clone)
}

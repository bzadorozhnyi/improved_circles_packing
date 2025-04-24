use std::sync::Arc;

use circles_pack::{
    calcfg::calcfg,
    circles_packing::CirclesPacking,
    dichotomy_step_ralgo::{dichotomy_step_ralgo, smart_dichotomy_step_ralgo},
    heuristic_algo::HeuristicAlgorithmBuilder,
    ralgo::RAlgorithmBuilder,
    utils::{measure_time, FloatType},
};
use nalgebra::DVector;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let total_iterations: usize = 1_000_000;
    let threads_number: usize = 10;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads_number)
        .build_global()
        .unwrap();

    let test_number = 40;
    let radiuses = (1..=test_number)
        .map(|x| x as FloatType)
        .collect::<Vec<_>>();

    let (total_time, best) = measure_time(|| {
        let par_iter = (1..=threads_number).into_par_iter().map(|x| {
            let thread_iterations = total_iterations / threads_number;
            let skip_iterations = (x - 1) * thread_iterations;

            let mut heuristic_algo = HeuristicAlgorithmBuilder::new()
                .iterations(thread_iterations)
                .radiuses(radiuses.clone())
                .max_small_circle_radius(
                    radiuses
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                        * 0.3,
                )
                .delta(1e-6)
                .build();

            heuristic_algo.find(Some(skip_iterations))
        });

        let answers: Vec<_> = par_iter.collect();

        let best = answers
            .into_iter()
            .filter(|a| a.is_valid_pack())
            .min_by(|a, b| {
                a.main_circle
                    .radius
                    .partial_cmp(&b.main_circle.radius)
                    .unwrap()
            })
            .unwrap();

        best
    });

    println!("iters = {total_iterations}");

    println!("heuristic");
    println!("time = {total_time}");
    println!("R = {}", best.main_circle.radius);
    best.plot("images/heuristic.png");

    println!();

    let x: DVector<FloatType> = best.into_coords_vec();
    let inner_circles_radiuses = best.inner_circles_radiuses();
    let inner_circles_radiuses_clone = inner_circles_radiuses.clone();

    let calcfg = move |x: &DVector<FloatType>| -> (FloatType, DVector<FloatType>) {
        calcfg(&x, &inner_circles_radiuses_clone)
    };

    let ralgo_base = RAlgorithmBuilder::new()
        .alpha(1.5)
        .max_iterations(100_000)
        .calcfg(Arc::new(calcfg));

    let (total_time_ralgo_1, improved_1) = measure_time(|| {
        let ralgo = ralgo_base.clone().q1(0.95).build();

        let x = dichotomy_step_ralgo(x.clone(), false, 0.0, ralgo);
        let improved_best_1 =
            CirclesPacking::from_coords_vec_and_radiuses(x, &inner_circles_radiuses);

        improved_best_1
    });

    let (total_time_ralgo_2, improved_2) = measure_time(|| {
        let ralgo = ralgo_base.clone().q1(1.0).build();

        let x = dichotomy_step_ralgo(x.clone(), false, 0.0, ralgo);
        let improved_best_2 =
            CirclesPacking::from_coords_vec_and_radiuses(x, &inner_circles_radiuses);

        improved_best_2
    });

    println!("heuristic + ralgo(q1=0.95)");
    println!("time = {total_time_ralgo_1}");
    println!("R = {}", improved_1.main_circle.radius);
    improved_1.plot("images/heuristic + ralgo(q1=0.95).png");

    println!();

    println!("heuristic + ralgo(q1=1.0)");
    println!("time = {total_time_ralgo_2}");
    println!("R = {}", improved_2.main_circle.radius);
    improved_2.plot("images/heuristic + ralgo(q1=1).png");

    println!();

    let (total_time_ralgo_1, improved_1) = measure_time(|| {
        let ralgo = ralgo_base.clone().q1(0.95).build();

        let x = smart_dichotomy_step_ralgo(x.clone(), 0.0, ralgo);
        let improved_best_1 =
            CirclesPacking::from_coords_vec_and_radiuses(x, &inner_circles_radiuses);

        improved_best_1
    });

    let (total_time_ralgo_2, improved_2) = measure_time(|| {
        let ralgo = ralgo_base.clone().q1(1.0).build();

        let x = smart_dichotomy_step_ralgo(x.clone(), 0.0, ralgo);
        let improved_best_2 =
            CirclesPacking::from_coords_vec_and_radiuses(x, &inner_circles_radiuses);

        improved_best_2
    });

    println!("(s) heuristic + ralgo(q1=0.95)");
    println!("time = {total_time_ralgo_1}");
    println!("R = {}", improved_1.main_circle.radius);
    improved_1.plot("images/heuristic + ralgo(q1=0.95) (s).png");

    println!();

    println!("(s) heuristic + ralgo(q1=1.0)");
    println!("time = {total_time_ralgo_2}");
    println!("R = {}", improved_2.main_circle.radius);
    improved_2.plot("images/heuristic + ralgo(q1=1) (s).png");
}

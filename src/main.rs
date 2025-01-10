use circles_pack::{
    dichotomy_step_ralgo::dichotomy_step_ralgo,
    heuristic_algo::HeuristicAlgorithmBuilder,
    ralgo::RAlgorithmBuilder,
    utils::{measure_time, FloatType},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let total_iterations: usize = 100_000;
    let threads_number: usize = 10;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads_number)
        .build_global()
        .unwrap();

    let test_number = 40;
    let radiuses = (1..=test_number)
        .map(|x| x as FloatType)
        .collect::<Vec<_>>();

    let (total_time, _) = measure_time(|| {
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
                        * 0.25,
                )
                .delta(1e-6)
                .build();

            heuristic_algo.find(Some(skip_iterations))
        });

        let answers: Vec<_> = par_iter.collect();

        for answer in &answers {
            println!("{} {}", answer.is_valid_pack(), answer.main_circle.radius)
        }

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

        println!("heuristic");
        best.print();

        let ralgo = RAlgorithmBuilder::new()
            .alpha(1.5)
            .q1(1.0)
            .max_iterations(100_000);

        let improved_best = dichotomy_step_ralgo(&best, false, 0.0, ralgo);
        println!("heuristic + ralgo");
        improved_best.print();
    });

    println!("{total_time}");
}

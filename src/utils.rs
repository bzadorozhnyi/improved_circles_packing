use std::time::Instant;

pub type FloatType = f64;

pub fn measure_time<F, T>(function: F) -> (FloatType, T)
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();
    let function_result = function();
    let end_time = Instant::now();

    let time = (end_time - start_time).as_secs_f64() as FloatType;

    (time, function_result)
}

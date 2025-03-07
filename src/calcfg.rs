use nalgebra::DVector;

use crate::utils::FloatType;

fn concat_gradients(
    gx: &DVector<FloatType>,
    gy: &DVector<FloatType>,
    gr: FloatType,
) -> DVector<FloatType> {
    let circles_number = gx.len();

    let mut gradient = DVector::<FloatType>::zeros(2 * circles_number + 1);
    gradient.rows_mut(0, circles_number).copy_from(&gx);
    gradient
        .rows_mut(circles_number, circles_number)
        .copy_from(&gy);
    gradient[2 * circles_number] = gr;

    return gradient;
}

pub fn calcfg(
    x: &DVector<FloatType>,
    radiuses: &Vec<FloatType>,
) -> (FloatType, DVector<FloatType>) {
    let number_of_circles = radiuses.len();

    let cx = x.rows(0, number_of_circles);
    let cy = x.rows(number_of_circles, number_of_circles);
    let main_circle_radius = x[x.len() - 1];

    let mut gx = DVector::<FloatType>::zeros(number_of_circles);
    let mut gy = DVector::<FloatType>::zeros(number_of_circles);
    let mut gr = 1 as FloatType;

    let mut f = main_circle_radius;
    const P1: FloatType = 2000.0;
    const P2: FloatType = 1000.0;
    const EPS: FloatType = 1e-24;

    for i in 0..number_of_circles {
        let (cx_i, cy_i) = (cx[i], cy[i]);
        let radius_diff = main_circle_radius - radiuses[i];

        let mut temp = cx_i * cx_i + cy_i * cy_i - radius_diff * radius_diff + EPS;
        if temp > 0.0 {
            f += P1 * temp;
            gx[i] += P1 * cx_i;
            gy[i] += P1 * cy_i;
            gr -= P2;
        }

        for j in (i + 1)..number_of_circles {
            let cx_diff = cx_i - cx[j];
            let cy_diff = cy_i - cy[j];
            let radius_sum = radiuses[i] + radiuses[j];

            temp = -(cx_diff * cx_diff + cy_diff * cy_diff) + radius_sum * radius_sum + EPS;
            if temp > 0.0 {
                f += P1 * temp;
                let scaled_cx = P1 * cx_diff;
                let scaled_cy = P1 * cy_diff;

                gx[i] -= scaled_cx;
                gy[i] -= scaled_cy;
                gx[j] += scaled_cx;
                gy[j] += scaled_cy;
            }
        }
    }

    let temp = -main_circle_radius
        + radiuses
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

    if temp > 0.0 {
        f += P2 * temp;
        gr -= P2;
    }

    (f, concat_gradients(&gx, &gy, gr))
}

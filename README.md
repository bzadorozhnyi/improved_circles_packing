# Circles packing

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![rayon](https://img.shields.io/badge/rayon-1.10.0-4B8BBE?logo=rust)](https://crates.io/crates/rayon)
[![nalgebra](https://img.shields.io/badge/nalgebra-0.33.2-FF6F61?logo=rust)](https://crates.io/crates/nalgebra)

Improved version of [heuristic packing algorithm](https://github.com/bzadorozhnyi/circles-packing) combined with dichotomy step ralgo. Check improvements and compare section for details.

## 📐 Condition

A set of small circles needed to be packed into a circle and placed in $(0, 0)$ so that any circles would not overlap and would be inside the main (the biggest) circle. The program finds one of the local minima of the problem of packing circles into a circle of the minimum radius.

$$
\begin{cases}
  (x_i-x_j)^2 + (y_i-y_j)^2 \ge (r_i+r_j)^2, &&\forall i,j (i \ne j) \\
  x_i^2+y_i^2 \le (R - r_i)^2, &&\forall i \\
\end{cases}
$$

$$
R \to \min
$$

Where $(x_i, y_i)$ is the coordinate of the given circle's center, $r_i$ - radius (same for $j$), $R$ - radius of the main circle.

## 🚀 Improvements

Compared to the original version, this implementation features:
- Improved code readability;
- Better performance;
- A more structured and maintainable architecture;
- General refactoring for clarity and flexibility.

## 🔁 Old VS New

For the test case: $$r_i = i, i = 1, ..., N$$:

<div align="center">

| Metric     | N=10     | N=20     | N=30     | N=40     | N=50     |
|------------|----------|----------|----------|----------|----------|
| $$Time_{old}$$   | 71.67 s  | 274.09 s | 645.80 s | 1196.87 s| 2004.54 s|
| $$R_{old}$$      | 22.56    | 59.95    | 108.13   | 165.10   | 230.41   |
| $$Time_{new}$$   | 3.94 s   | 15.68 s  | 39.65 s  | 77.66 s  | 131.99 s |
| $$R_{new}$$      | 22.00    | 58.74    | 106.39   | 162.28   | 225.33   |

</div>

✅ Average performance gain: ~15× faster

✅ Average packing quality gain: slightly tighter (smaller radius)

## 🧪 Examples

Possible solution for the test case: $$r_i = i, i = 1, ..., 40$$:

### Heuristic:

🕒 Time: 11.856300403 s

🟢 Radius: 162.28294008166233

<img alt="heuristic" src="https://github.com/user-attachments/assets/989e8372-96a3-4df8-b4ad-d4ccd19bc70d" />

### Heuristic + ralgo (q1=0.95) (dichotomy step ralgo)

🕒 Time = 2.110014565 s

🟢 R = 162.20409989758096

<img alt="heuristic + ralgo(q1=0 95)" src="https://github.com/user-attachments/assets/087e5257-1404-4d5d-a34c-f5359a838845" />

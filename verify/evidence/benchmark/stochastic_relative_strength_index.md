# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.47M | 0.022 | 44.50M | 0.063 | 2.72× | 2.78× |
| 10,000 | 0.197 | 50.84M | 0.206 | 48.58M | 0.207 | 1.05× | 1.01× |
| 100,000 | 2.663 | 37.55M | 2.520 | 39.69M | 1.631 | 0.61× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.154 | 1.90× |
| 1 | 5 | 0.238 | 0.554 | 2.33× |
| 1 | 10 | 0.404 | 1.081 | 2.68× |
| 10 | 1 | 0.045 | 0.123 | 2.72× |
| 10 | 5 | 0.199 | 0.524 | 2.63× |
| 10 | 10 | 0.411 | 1.082 | 2.63× |
| 100 | 1 | 0.046 | 0.106 | 2.33× |
| 100 | 5 | 0.194 | 0.561 | 2.89× |
| 100 | 10 | 0.430 | 1.081 | 2.51× |
| 1,000 | 1 | 0.066 | 0.121 | 1.83× |
| 1,000 | 5 | 0.209 | 0.605 | 2.89× |
| 1,000 | 10 | 0.471 | 1.239 | 2.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

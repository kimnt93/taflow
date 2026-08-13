# ExponentiallyWeightedVariance benchmark (`ewm variance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.43M | 0.021 | 46.64M | 1.221 | 45.71× | 56.96× |
| 10,000 | 0.155 | 64.71M | 0.144 | 69.54M | 13.599 | 88.00× | 94.57× |
| 100,000 | 1.392 | 71.83M | 1.369 | 73.04M | 116.535 | 83.70× | 85.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.194 | 0.121 | 0.63× |
| 1 | 5 | 0.496 | 0.450 | 0.91× |
| 1 | 10 | 0.579 | 0.970 | 1.68× |
| 10 | 1 | 0.077 | 0.095 | 1.22× |
| 10 | 5 | 0.284 | 0.485 | 1.71× |
| 10 | 10 | 0.592 | 0.930 | 1.57× |
| 100 | 1 | 0.062 | 0.203 | 3.29× |
| 100 | 5 | 0.279 | 0.997 | 3.57× |
| 100 | 10 | 0.579 | 2.020 | 3.49× |
| 1,000 | 1 | 0.076 | 1.318 | 17.26× |
| 1,000 | 5 | 0.294 | 6.613 | 22.47× |
| 1,000 | 10 | 0.649 | 14.298 | 22.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

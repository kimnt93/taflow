# CloseToCloseSigma benchmark (`annualized close-to-close volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.13M | 0.021 | 47.66M | 0.140 | 6.47× | 6.69× |
| 10,000 | 0.188 | 53.08M | 0.180 | 55.49M | 0.657 | 3.49× | 3.64× |
| 100,000 | 2.002 | 49.96M | 1.829 | 54.67M | 6.934 | 3.46× | 3.79× |
| 1,000,000 | 18.235 | 54.84M | 18.098 | 55.25M | 72.298 | 3.96× | 3.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.142 | 1.59× |
| 1 | 5 | 0.264 | 0.904 | 3.43× |
| 1 | 10 | 0.481 | 1.215 | 2.53× |
| 10 | 1 | 0.055 | 0.116 | 2.12× |
| 10 | 5 | 0.252 | 0.634 | 2.52× |
| 10 | 10 | 0.533 | 1.226 | 2.30× |
| 100 | 1 | 0.054 | 0.178 | 3.33× |
| 100 | 5 | 0.264 | 0.848 | 3.21× |
| 100 | 10 | 0.523 | 1.878 | 3.59× |
| 1,000 | 1 | 0.076 | 0.234 | 3.06× |
| 1,000 | 5 | 0.262 | 1.103 | 4.20× |
| 1,000 | 10 | 0.520 | 2.289 | 4.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

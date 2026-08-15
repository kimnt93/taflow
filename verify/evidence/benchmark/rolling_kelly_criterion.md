# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.30M | 0.019 | 52.67M | 0.182 | 8.97× | 9.58× |
| 10,000 | 0.190 | 52.73M | 0.196 | 51.12M | 0.699 | 3.69× | 3.57× |
| 100,000 | 1.843 | 54.27M | 1.868 | 53.55M | 5.834 | 3.17× | 3.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.239 | 3.52× |
| 1 | 5 | 0.222 | 1.032 | 4.65× |
| 1 | 10 | 0.468 | 2.173 | 4.64× |
| 10 | 1 | 0.045 | 0.193 | 4.25× |
| 10 | 5 | 0.186 | 1.022 | 5.51× |
| 10 | 10 | 0.457 | 2.220 | 4.86× |
| 100 | 1 | 0.048 | 0.237 | 4.99× |
| 100 | 5 | 0.263 | 1.050 | 3.99× |
| 100 | 10 | 0.460 | 2.329 | 5.07× |
| 1,000 | 1 | 0.075 | 0.300 | 4.00× |
| 1,000 | 5 | 0.226 | 1.262 | 5.58× |
| 1,000 | 10 | 0.502 | 2.897 | 5.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

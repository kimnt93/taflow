# MovingAverageEnvelope benchmark (`MaEnvelope` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.61M | 0.021 | 46.93M | 0.532 | 22.13× | 24.96× |
| 10,000 | 0.211 | 47.50M | 0.190 | 52.69M | 4.479 | 21.27× | 23.60× |
| 100,000 | 2.019 | 49.54M | 1.840 | 54.36M | 41.295 | 20.46× | 22.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.315 | 4.31× |
| 1 | 5 | 0.268 | 1.349 | 5.04× |
| 1 | 10 | 0.508 | 2.748 | 5.41× |
| 10 | 1 | 0.051 | 0.256 | 5.02× |
| 10 | 5 | 0.228 | 1.421 | 6.24× |
| 10 | 10 | 0.505 | 2.798 | 5.54× |
| 100 | 1 | 0.052 | 0.289 | 5.60× |
| 100 | 5 | 0.252 | 1.575 | 6.24× |
| 100 | 10 | 0.509 | 3.201 | 6.28× |
| 1,000 | 1 | 0.071 | 0.811 | 11.41× |
| 1,000 | 5 | 0.249 | 3.404 | 13.67× |
| 1,000 | 10 | 0.502 | 6.829 | 13.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

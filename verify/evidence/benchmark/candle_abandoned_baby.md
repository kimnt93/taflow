# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.99M | 0.019 | 53.26M | 0.042 | 1.84× | 2.23× |
| 10,000 | 0.193 | 51.78M | 0.173 | 57.78M | 0.151 | 0.78× | 0.87× |
| 100,000 | 1.831 | 54.61M | 1.867 | 53.56M | 1.294 | 0.71× | 0.69× |
| 1,000,000 | 18.551 | 53.90M | 19.130 | 52.27M | 11.825 | 0.64× | 0.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.151 | 1.73× |
| 1 | 5 | 0.328 | 0.531 | 1.62× |
| 1 | 10 | 0.638 | 1.085 | 1.70× |
| 10 | 1 | 0.065 | 0.114 | 1.73× |
| 10 | 5 | 0.291 | 0.581 | 1.99× |
| 10 | 10 | 0.583 | 1.120 | 1.92× |
| 100 | 1 | 0.081 | 0.102 | 1.26× |
| 100 | 5 | 0.336 | 0.540 | 1.61× |
| 100 | 10 | 0.625 | 1.153 | 1.85× |
| 1,000 | 1 | 0.095 | 0.117 | 1.22× |
| 1,000 | 5 | 0.364 | 0.573 | 1.58× |
| 1,000 | 10 | 0.677 | 1.164 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

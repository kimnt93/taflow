# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.29M | 0.007 | 139.15M | 0.038 | 3.97× | 5.24× |
| 10,000 | 0.023 | 426.44M | 0.020 | 488.82M | 0.043 | 1.82× | 2.09× |
| 100,000 | 0.202 | 495.25M | 0.176 | 569.70M | 0.139 | 0.69× | 0.79× |
| 1,000,000 | 2.511 | 398.26M | 2.004 | 498.90M | 1.277 | 0.51× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.125 | 1.27× |
| 1 | 5 | 0.302 | 0.516 | 1.71× |
| 1 | 10 | 0.501 | 1.088 | 2.17× |
| 10 | 1 | 0.050 | 0.099 | 2.00× |
| 10 | 5 | 0.289 | 0.549 | 1.90× |
| 10 | 10 | 0.519 | 1.030 | 1.99× |
| 100 | 1 | 0.054 | 0.095 | 1.76× |
| 100 | 5 | 0.255 | 0.481 | 1.89× |
| 100 | 10 | 0.583 | 1.026 | 1.76× |
| 1,000 | 1 | 0.058 | 0.109 | 1.88× |
| 1,000 | 5 | 0.246 | 0.519 | 2.11× |
| 1,000 | 10 | 0.608 | 1.122 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

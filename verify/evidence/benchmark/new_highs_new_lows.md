# NewHighsNewLows benchmark (`NewHighsNewLows` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.86M | 0.006 | 171.52M | 8.266 | 1172.61× | 1417.81× |
| 10,000 | 0.029 | 346.12M | 0.026 | 383.50M | 78.407 | 2713.84× | 3006.90× |
| 100,000 | 0.241 | 415.71M | 0.213 | 469.24M | 830.667 | 3453.17× | 3897.83× |
| 1,000,000 | 3.100 | 322.61M | 3.544 | 282.17M | 8265.390 | 2666.53× | 2332.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.284 | 3.61× |
| 1 | 5 | 0.381 | 1.345 | 3.53× |
| 1 | 10 | 0.463 | 2.235 | 4.82× |
| 10 | 1 | 0.053 | 0.288 | 5.45× |
| 10 | 5 | 0.224 | 1.407 | 6.30× |
| 10 | 10 | 0.459 | 3.025 | 6.59× |
| 100 | 1 | 0.060 | 1.048 | 17.62× |
| 100 | 5 | 0.230 | 5.265 | 22.88× |
| 100 | 10 | 0.500 | 10.810 | 21.60× |
| 1,000 | 1 | 0.056 | 8.435 | 150.39× |
| 1,000 | 5 | 0.464 | 46.293 | 99.85× |
| 1,000 | 10 | 0.632 | 91.766 | 145.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

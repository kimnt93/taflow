# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.57M | 0.067 | 14.91M | 0.078 | 1.13× | 1.16× |
| 10,000 | 0.659 | 15.19M | 0.671 | 14.91M | 0.578 | 0.88× | 0.86× |
| 100,000 | 6.696 | 14.93M | 7.092 | 14.10M | 6.024 | 0.90× | 0.85× |
| 1,000,000 | 70.243 | 14.24M | 71.893 | 13.91M | 64.983 | 0.93× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.105 | 0.83× |
| 1 | 5 | 0.380 | 0.429 | 1.13× |
| 1 | 10 | 0.448 | 0.913 | 2.04× |
| 10 | 1 | 0.046 | 0.087 | 1.89× |
| 10 | 5 | 0.208 | 0.411 | 1.98× |
| 10 | 10 | 0.445 | 0.853 | 1.92× |
| 100 | 1 | 0.056 | 0.098 | 1.76× |
| 100 | 5 | 0.214 | 0.448 | 2.10× |
| 100 | 10 | 0.474 | 0.938 | 1.98× |
| 1,000 | 1 | 0.134 | 0.151 | 1.13× |
| 1,000 | 5 | 0.255 | 0.732 | 2.87× |
| 1,000 | 10 | 0.548 | 1.524 | 2.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

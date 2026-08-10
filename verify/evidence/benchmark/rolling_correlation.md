# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.49M | 0.009 | 107.25M | 0.046 | 3.75× | 4.88× |
| 10,000 | 0.089 | 112.54M | 0.087 | 114.58M | 0.096 | 1.08× | 1.10× |
| 100,000 | 0.593 | 168.53M | 0.528 | 189.53M | 0.627 | 1.06× | 1.19× |
| 1,000,000 | 5.714 | 175.01M | 5.218 | 191.63M | 5.986 | 1.05× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.116 | 0.84× |
| 1 | 5 | 0.352 | 0.552 | 1.57× |
| 1 | 10 | 0.616 | 0.994 | 1.61× |
| 10 | 1 | 0.047 | 0.097 | 2.03× |
| 10 | 5 | 0.287 | 0.703 | 2.45× |
| 10 | 10 | 0.563 | 1.022 | 1.82× |
| 100 | 1 | 0.049 | 0.096 | 1.96× |
| 100 | 5 | 0.241 | 0.523 | 2.17× |
| 100 | 10 | 0.586 | 0.999 | 1.70× |
| 1,000 | 1 | 0.055 | 0.102 | 1.86× |
| 1,000 | 5 | 0.283 | 0.501 | 1.77× |
| 1,000 | 10 | 0.630 | 1.116 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

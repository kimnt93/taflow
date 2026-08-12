# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.27M | 0.013 | 79.10M | 0.033 | 2.31× | 2.64× |
| 10,000 | 0.110 | 90.78M | 0.107 | 93.39M | 0.090 | 0.82× | 0.84× |
| 100,000 | 1.094 | 91.43M | 1.070 | 93.49M | 0.607 | 0.56× | 0.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.106 | 0.89× |
| 1 | 5 | 0.266 | 0.436 | 1.64× |
| 1 | 10 | 0.482 | 0.899 | 1.86× |
| 10 | 1 | 0.049 | 0.089 | 1.82× |
| 10 | 5 | 0.264 | 0.465 | 1.76× |
| 10 | 10 | 0.507 | 0.902 | 1.78× |
| 100 | 1 | 0.050 | 0.088 | 1.75× |
| 100 | 5 | 0.247 | 0.445 | 1.80× |
| 100 | 10 | 0.562 | 0.936 | 1.67× |
| 1,000 | 1 | 0.066 | 0.098 | 1.50× |
| 1,000 | 5 | 0.240 | 0.484 | 2.02× |
| 1,000 | 10 | 0.518 | 1.078 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

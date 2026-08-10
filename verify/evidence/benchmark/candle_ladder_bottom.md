# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.69M | 0.017 | 60.60M | 0.034 | 1.68× | 2.09× |
| 10,000 | 0.136 | 73.66M | 0.130 | 76.81M | 0.085 | 0.63× | 0.66× |
| 100,000 | 1.401 | 71.40M | 1.440 | 69.47M | 0.601 | 0.43× | 0.42× |
| 1,000,000 | 13.822 | 72.35M | 13.757 | 72.69M | 6.096 | 0.44× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.120 | 1.27× |
| 1 | 5 | 0.321 | 0.436 | 1.36× |
| 1 | 10 | 0.494 | 0.864 | 1.75× |
| 10 | 1 | 0.055 | 0.090 | 1.62× |
| 10 | 5 | 0.242 | 0.457 | 1.89× |
| 10 | 10 | 0.550 | 0.914 | 1.66× |
| 100 | 1 | 0.058 | 0.097 | 1.68× |
| 100 | 5 | 0.252 | 0.432 | 1.71× |
| 100 | 10 | 0.578 | 1.017 | 1.76× |
| 1,000 | 1 | 0.070 | 0.096 | 1.38× |
| 1,000 | 5 | 0.276 | 0.448 | 1.62× |
| 1,000 | 10 | 0.585 | 1.077 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# Squeeze benchmark (`squeeze` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.22M | 0.026 | 38.57M | 4.241 | 132.40× | 163.55× |
| 10,000 | 0.237 | 42.20M | 0.229 | 43.71M | 6.210 | 26.20× | 27.15× |
| 100,000 | 2.490 | 40.16M | 2.343 | 42.68M | 25.459 | 10.22× | 10.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | 0.359 | 2.13× |
| 1 | 5 | 0.239 | 1.536 | 6.43× |
| 1 | 10 | 0.417 | 2.915 | 6.99× |
| 10 | 1 | 0.045 | 0.301 | 6.66× |
| 10 | 5 | 0.214 | 1.459 | 6.83× |
| 10 | 10 | 0.427 | 2.881 | 6.75× |
| 100 | 1 | 0.049 | 4.668 | 94.46× |
| 100 | 5 | 0.237 | 23.388 | 98.76× |
| 100 | 10 | 0.463 | 47.049 | 101.53× |
| 1,000 | 1 | 0.076 | 4.827 | 63.52× |
| 1,000 | 5 | 0.381 | 25.431 | 66.72× |
| 1,000 | 10 | 0.474 | 51.073 | 107.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

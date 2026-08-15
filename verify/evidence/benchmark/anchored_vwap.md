# AnchoredVolumeWeightedAveragePrice benchmark (`anchored VWAP deviation bands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.01M | 0.008 | 126.11M | 1.353 | 117.76× | 170.66× |
| 10,000 | 0.077 | 129.48M | 0.068 | 146.87M | 13.583 | 175.87× | 199.48× |
| 100,000 | 0.776 | 128.79M | 0.653 | 153.14M | 132.748 | 170.96× | 203.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.146 | 1.33× |
| 1 | 5 | 0.316 | 0.463 | 1.46× |
| 1 | 10 | 0.397 | 0.917 | 2.31× |
| 10 | 1 | 0.042 | 0.108 | 2.55× |
| 10 | 5 | 0.203 | 0.549 | 2.70× |
| 10 | 10 | 0.397 | 1.047 | 2.64× |
| 100 | 1 | 0.046 | 0.239 | 5.18× |
| 100 | 5 | 0.196 | 1.224 | 6.23× |
| 100 | 10 | 0.425 | 2.406 | 5.66× |
| 1,000 | 1 | 0.054 | 1.601 | 29.43× |
| 1,000 | 5 | 0.238 | 7.614 | 32.01× |
| 1,000 | 10 | 0.486 | 14.920 | 30.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# AnchoredVolumeWeightedAveragePrice benchmark (`anchored VWAP deviation bands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.66M | 0.013 | 74.47M | 1.305 | 72.62× | 97.18× |
| 10,000 | 0.088 | 114.10M | 0.078 | 128.72M | 12.950 | 147.76× | 166.69× |
| 100,000 | 0.834 | 119.93M | 0.779 | 128.34M | 126.569 | 151.80× | 162.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.121 | 1.38× |
| 1 | 5 | 0.322 | 0.463 | 1.44× |
| 1 | 10 | 0.487 | 0.948 | 1.95× |
| 10 | 1 | 0.058 | 0.108 | 1.85× |
| 10 | 5 | 0.239 | 0.546 | 2.29× |
| 10 | 10 | 0.533 | 1.080 | 2.03× |
| 100 | 1 | 0.053 | 0.240 | 4.50× |
| 100 | 5 | 0.245 | 1.138 | 4.64× |
| 100 | 10 | 0.539 | 2.312 | 4.29× |
| 1,000 | 1 | 0.066 | 1.458 | 22.07× |
| 1,000 | 5 | 0.250 | 7.370 | 29.47× |
| 1,000 | 10 | 0.585 | 15.817 | 27.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

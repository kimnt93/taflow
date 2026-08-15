# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 456.13M | 0.001 | 737.65M | 0.029 | 13.19× | 21.33× |
| 10,000 | 0.008 | 1.27G | 0.006 | 1.80G | 0.038 | 4.76× | 6.78× |
| 100,000 | 0.068 | 1.47G | 0.048 | 2.09G | 0.117 | 1.72× | 2.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.150 | 2.29× |
| 1 | 5 | 0.406 | 0.479 | 1.18× |
| 1 | 10 | 0.391 | 0.904 | 2.31× |
| 10 | 1 | 0.044 | 0.086 | 1.96× |
| 10 | 5 | 0.184 | 0.421 | 2.29× |
| 10 | 10 | 0.391 | 0.919 | 2.35× |
| 100 | 1 | 0.042 | 0.089 | 2.12× |
| 100 | 5 | 0.177 | 0.439 | 2.47× |
| 100 | 10 | 0.387 | 0.891 | 2.30× |
| 1,000 | 1 | 0.038 | 0.092 | 2.42× |
| 1,000 | 5 | 0.187 | 0.422 | 2.25× |
| 1,000 | 10 | 0.400 | 0.947 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

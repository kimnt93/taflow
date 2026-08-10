# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.98M | 0.018 | 54.79M | 0.042 | 1.94× | 2.31× |
| 10,000 | 0.132 | 75.59M | 0.124 | 80.74M | 0.101 | 0.76× | 0.82× |
| 100,000 | 1.242 | 80.52M | 1.168 | 85.59M | 0.776 | 0.62× | 0.66× |
| 1,000,000 | 13.069 | 76.52M | 12.202 | 81.95M | 7.222 | 0.55× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.150 | 1.47× |
| 1 | 5 | 0.347 | 0.595 | 1.72× |
| 1 | 10 | 0.771 | 1.366 | 1.77× |
| 10 | 1 | 0.071 | 0.108 | 1.53× |
| 10 | 5 | 0.357 | 0.707 | 1.98× |
| 10 | 10 | 0.831 | 1.332 | 1.60× |
| 100 | 1 | 0.068 | 0.102 | 1.50× |
| 100 | 5 | 0.381 | 0.684 | 1.80× |
| 100 | 10 | 5.131 | 1.477 | 0.29× |
| 1,000 | 1 | 0.095 | 0.151 | 1.59× |
| 1,000 | 5 | 0.428 | 0.644 | 1.51× |
| 1,000 | 10 | 0.874 | 1.554 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

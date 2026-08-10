# TradeVolumeIndex benchmark (`TradeVolumeIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.67M | 0.007 | 143.16M | 0.213 | 26.17× | 30.54× |
| 10,000 | 0.076 | 131.17M | 0.072 | 138.24M | 0.826 | 10.83× | 11.41× |
| 100,000 | 0.743 | 134.66M | 0.714 | 140.03M | 7.235 | 9.74× | 10.13× |
| 1,000,000 | 7.552 | 132.42M | 6.934 | 144.21M | 72.347 | 9.58× | 10.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.264 | 3.29× |
| 1 | 5 | 0.400 | 1.457 | 3.65× |
| 1 | 10 | 0.528 | 2.367 | 4.48× |
| 10 | 1 | 0.055 | 0.214 | 3.92× |
| 10 | 5 | 0.274 | 1.320 | 4.81× |
| 10 | 10 | 0.546 | 2.356 | 4.32× |
| 100 | 1 | 0.057 | 0.225 | 3.96× |
| 100 | 5 | 0.283 | 1.310 | 4.62× |
| 100 | 10 | 0.529 | 2.527 | 4.77× |
| 1,000 | 1 | 0.065 | 0.286 | 4.37× |
| 1,000 | 5 | 0.273 | 1.629 | 5.97× |
| 1,000 | 10 | 0.599 | 3.102 | 5.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

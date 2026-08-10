# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.14M | 0.008 | 128.41M | 0.039 | 4.48× | 5.04× |
| 10,000 | 0.061 | 162.79M | 0.060 | 166.82M | 0.100 | 1.63× | 1.67× |
| 100,000 | 0.580 | 172.56M | 0.556 | 179.80M | 0.637 | 1.10× | 1.14× |
| 1,000,000 | 6.448 | 155.09M | 6.048 | 165.35M | 5.958 | 0.92× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.170 | 1.83× |
| 1 | 5 | 0.395 | 0.490 | 1.24× |
| 1 | 10 | 0.457 | 0.930 | 2.03× |
| 10 | 1 | 0.049 | 0.090 | 1.82× |
| 10 | 5 | 0.225 | 0.480 | 2.13× |
| 10 | 10 | 0.518 | 0.988 | 1.91× |
| 100 | 1 | 0.050 | 0.087 | 1.74× |
| 100 | 5 | 0.211 | 0.481 | 2.28× |
| 100 | 10 | 0.483 | 0.966 | 2.00× |
| 1,000 | 1 | 0.055 | 0.101 | 1.84× |
| 1,000 | 5 | 0.218 | 0.475 | 2.18× |
| 1,000 | 10 | 0.473 | 1.007 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

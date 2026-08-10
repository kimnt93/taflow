# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.34M | 0.007 | 134.60M | 0.029 | 2.80× | 3.87× |
| 10,000 | 0.069 | 145.28M | 0.073 | 136.41M | 0.082 | 1.19× | 1.12× |
| 100,000 | 0.718 | 139.27M | 0.709 | 141.00M | 0.560 | 0.78× | 0.79× |
| 1,000,000 | 7.371 | 135.67M | 7.221 | 138.49M | 5.395 | 0.73× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.133 | 1.11× |
| 1 | 5 | 0.368 | 0.504 | 1.37× |
| 1 | 10 | 0.530 | 0.909 | 1.72× |
| 10 | 1 | 0.067 | 0.089 | 1.34× |
| 10 | 5 | 0.240 | 0.434 | 1.81× |
| 10 | 10 | 0.548 | 0.913 | 1.66× |
| 100 | 1 | 0.054 | 0.090 | 1.67× |
| 100 | 5 | 0.248 | 0.428 | 1.73× |
| 100 | 10 | 0.541 | 0.933 | 1.72× |
| 1,000 | 1 | 0.061 | 0.092 | 1.50× |
| 1,000 | 5 | 0.264 | 0.466 | 1.77× |
| 1,000 | 10 | 0.554 | 0.979 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

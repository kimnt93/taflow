# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.72M | 0.007 | 150.18M | 0.038 | 4.75× | 5.67× |
| 10,000 | 0.069 | 144.94M | 0.067 | 149.40M | 0.097 | 1.41× | 1.45× |
| 100,000 | 0.778 | 128.48M | 0.678 | 147.52M | 0.678 | 0.87× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.114 | 1.52× |
| 1 | 5 | 0.216 | 0.465 | 2.15× |
| 1 | 10 | 0.502 | 1.429 | 2.85× |
| 10 | 1 | 0.067 | 0.132 | 1.97× |
| 10 | 5 | 0.293 | 0.616 | 2.10× |
| 10 | 10 | 0.541 | 1.488 | 2.75× |
| 100 | 1 | 0.066 | 0.142 | 2.16× |
| 100 | 5 | 0.265 | 0.628 | 2.37× |
| 100 | 10 | 0.512 | 1.294 | 2.53× |
| 1,000 | 1 | 0.061 | 0.109 | 1.79× |
| 1,000 | 5 | 0.279 | 0.543 | 1.95× |
| 1,000 | 10 | 0.527 | 1.049 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

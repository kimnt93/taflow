# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.01M | 0.008 | 119.52M | 0.035 | 3.73× | 4.20× |
| 10,000 | 0.072 | 139.53M | 0.067 | 149.05M | 0.097 | 1.36× | 1.45× |
| 100,000 | 0.712 | 140.48M | 0.682 | 146.58M | 0.690 | 0.97× | 1.01× |
| 1,000,000 | 7.545 | 132.55M | 7.087 | 141.10M | 6.904 | 0.92× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.115 | 1.65× |
| 1 | 5 | 0.349 | 0.560 | 1.61× |
| 1 | 10 | 0.564 | 1.003 | 1.78× |
| 10 | 1 | 0.057 | 0.097 | 1.71× |
| 10 | 5 | 0.244 | 0.436 | 1.79× |
| 10 | 10 | 0.480 | 0.964 | 2.01× |
| 100 | 1 | 0.057 | 0.103 | 1.80× |
| 100 | 5 | 0.268 | 0.456 | 1.70× |
| 100 | 10 | 0.527 | 1.024 | 1.94× |
| 1,000 | 1 | 0.068 | 0.122 | 1.79× |
| 1,000 | 5 | 0.321 | 0.602 | 1.87× |
| 1,000 | 10 | 0.621 | 1.014 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

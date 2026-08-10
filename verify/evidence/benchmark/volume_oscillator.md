# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 74.07M | 0.013 | 76.69M | 0.206 | 15.28× | 15.82× |
| 10,000 | 0.110 | 90.63M | 0.108 | 92.83M | 0.558 | 5.06× | 5.18× |
| 100,000 | 1.085 | 92.18M | 1.194 | 83.78M | 4.063 | 3.75× | 3.40× |
| 1,000,000 | 10.931 | 91.48M | 10.321 | 96.89M | 39.021 | 3.57× | 3.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.334 | 2.55× |
| 1 | 5 | 0.266 | 1.218 | 4.57× |
| 1 | 10 | 0.495 | 2.574 | 5.20× |
| 10 | 1 | 0.051 | 0.244 | 4.77× |
| 10 | 5 | 0.255 | 1.407 | 5.52× |
| 10 | 10 | 0.516 | 2.730 | 5.29× |
| 100 | 1 | 0.055 | 0.241 | 4.37× |
| 100 | 5 | 0.232 | 1.374 | 5.92× |
| 100 | 10 | 0.480 | 2.574 | 5.37× |
| 1,000 | 1 | 0.063 | 0.294 | 4.68× |
| 1,000 | 5 | 0.238 | 1.616 | 6.80× |
| 1,000 | 10 | 0.519 | 3.284 | 6.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

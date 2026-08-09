# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.148 | 6.78M | 0.143 | 6.99M | 0.484 | 3.28× | 3.39× |
| 10,000 | 1.345 | 7.44M | 1.342 | 7.45M | 4.370 | 3.25× | 3.26× |
| 100,000 | 13.675 | 7.31M | 13.718 | 7.29M | 44.292 | 3.24× | 3.23× |
| 1,000,000 | 133.911 | 7.47M | 136.339 | 7.33M | 460.883 | 3.44× | 3.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.167 | 1.55× |
| 1 | 5 | 0.272 | 0.526 | 1.93× |
| 1 | 10 | 0.528 | 1.130 | 2.14× |
| 10 | 1 | 0.054 | 0.098 | 1.81× |
| 10 | 5 | 0.240 | 0.522 | 2.18× |
| 10 | 10 | 0.550 | 1.052 | 1.91× |
| 100 | 1 | 0.075 | 0.141 | 1.88× |
| 100 | 5 | 0.273 | 0.657 | 2.41× |
| 100 | 10 | 0.553 | 1.379 | 2.49× |
| 1,000 | 1 | 0.205 | 0.629 | 3.06× |
| 1,000 | 5 | 0.396 | 3.000 | 7.57× |
| 1,000 | 10 | 0.665 | 6.208 | 9.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

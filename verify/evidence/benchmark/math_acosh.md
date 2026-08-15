# MathAcosh benchmark (`numpy.arccosh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.91M | 0.010 | 95.36M | 0.022 | 1.89× | 2.13× |
| 10,000 | 0.109 | 91.71M | 0.102 | 97.99M | 0.116 | 1.06× | 1.13× |
| 100,000 | 1.032 | 96.86M | 1.038 | 96.31M | 1.016 | 0.98× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.136 | 1.56× |
| 1 | 5 | 0.298 | 0.315 | 1.06× |
| 1 | 10 | 0.384 | 0.566 | 1.48× |
| 10 | 1 | 0.040 | 0.055 | 1.38× |
| 10 | 5 | 0.194 | 0.278 | 1.43× |
| 10 | 10 | 0.368 | 0.622 | 1.69× |
| 100 | 1 | 0.045 | 0.064 | 1.41× |
| 100 | 5 | 0.187 | 0.276 | 1.47× |
| 100 | 10 | 0.410 | 0.579 | 1.41× |
| 1,000 | 1 | 0.054 | 0.067 | 1.23× |
| 1,000 | 5 | 0.198 | 0.346 | 1.75× |
| 1,000 | 10 | 0.442 | 0.797 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

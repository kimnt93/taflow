# VolumeZoneOscillator benchmark (`VZO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.04M | 0.058 | 17.14M | 0.199 | 2.99× | 3.40× |
| 10,000 | 0.499 | 20.04M | 0.489 | 20.46M | 0.847 | 1.70× | 1.73× |
| 100,000 | 5.095 | 19.63M | 4.822 | 20.74M | 7.197 | 1.41× | 1.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.224 | 1.92× |
| 1 | 5 | 0.418 | 1.103 | 2.64× |
| 1 | 10 | 1.092 | 2.250 | 2.06× |
| 10 | 1 | 0.076 | 0.213 | 2.79× |
| 10 | 5 | 0.316 | 1.225 | 3.87× |
| 10 | 10 | 0.688 | 2.276 | 3.31× |
| 100 | 1 | 0.077 | 0.229 | 2.96× |
| 100 | 5 | 0.320 | 1.307 | 4.08× |
| 100 | 10 | 0.792 | 2.402 | 3.03× |
| 1,000 | 1 | 0.136 | 0.291 | 2.13× |
| 1,000 | 5 | 0.320 | 1.665 | 5.21× |
| 1,000 | 10 | 0.692 | 3.014 | 4.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

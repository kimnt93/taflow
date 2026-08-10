# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.81M | 0.013 | 77.70M | 0.094 | 7.16× | 7.34× |
| 10,000 | 0.105 | 94.89M | 0.102 | 98.10M | 0.727 | 6.90× | 7.13× |
| 100,000 | 0.997 | 100.28M | 0.965 | 103.67M | 6.905 | 6.92× | 7.16× |
| 1,000,000 | 10.452 | 95.68M | 9.992 | 100.08M | 88.853 | 8.50× | 8.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.159 | 1.61× |
| 1 | 5 | 0.328 | 0.398 | 1.22× |
| 1 | 10 | 0.473 | 0.779 | 1.65× |
| 10 | 1 | 0.049 | 0.078 | 1.60× |
| 10 | 5 | 0.241 | 0.399 | 1.65× |
| 10 | 10 | 0.504 | 0.826 | 1.64× |
| 100 | 1 | 0.055 | 0.087 | 1.60× |
| 100 | 5 | 0.224 | 0.410 | 1.83× |
| 100 | 10 | 0.474 | 0.910 | 1.92× |
| 1,000 | 1 | 0.067 | 0.185 | 2.77× |
| 1,000 | 5 | 0.227 | 0.730 | 3.22× |
| 1,000 | 10 | 0.498 | 1.523 | 3.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

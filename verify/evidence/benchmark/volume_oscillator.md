# VolumeOscillator benchmark (`VolumeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.72M | 0.013 | 79.67M | 0.208 | 16.58× | 16.56× |
| 10,000 | 0.106 | 94.67M | 0.101 | 99.16M | 0.614 | 5.81× | 6.09× |
| 100,000 | 1.080 | 92.63M | 1.077 | 92.87M | 4.008 | 3.71× | 3.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.295 | 3.66× |
| 1 | 5 | 0.294 | 1.417 | 4.82× |
| 1 | 10 | 0.408 | 2.638 | 6.46× |
| 10 | 1 | 0.048 | 0.237 | 4.91× |
| 10 | 5 | 0.221 | 1.421 | 6.44× |
| 10 | 10 | 0.408 | 2.502 | 6.13× |
| 100 | 1 | 0.054 | 0.243 | 4.53× |
| 100 | 5 | 0.227 | 1.390 | 6.11× |
| 100 | 10 | 0.414 | 2.764 | 6.68× |
| 1,000 | 1 | 0.057 | 0.277 | 4.84× |
| 1,000 | 5 | 0.213 | 1.567 | 7.37× |
| 1,000 | 10 | 0.452 | 2.997 | 6.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

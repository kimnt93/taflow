# VolumeByTimeProfile benchmark (`VolumeByTimeProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.07M | 0.045 | 22.29M | 1.569 | 28.35× | 34.97× |
| 10,000 | 0.492 | 20.32M | 0.427 | 23.41M | 15.671 | 31.84× | 36.69× |
| 100,000 | 5.778 | 17.31M | 4.421 | 22.62M | 184.962 | 32.01× | 41.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.342 | 3.80× |
| 1 | 5 | 0.223 | 1.455 | 6.53× |
| 1 | 10 | 0.433 | 2.793 | 6.45× |
| 10 | 1 | 0.047 | 0.265 | 5.60× |
| 10 | 5 | 0.210 | 1.515 | 7.21× |
| 10 | 10 | 0.452 | 2.802 | 6.20× |
| 100 | 1 | 0.056 | 0.390 | 6.90× |
| 100 | 5 | 0.196 | 2.306 | 11.76× |
| 100 | 10 | 0.431 | 4.291 | 9.95× |
| 1,000 | 1 | 0.096 | 1.922 | 20.03× |
| 1,000 | 5 | 0.210 | 9.724 | 46.34× |
| 1,000 | 10 | 0.526 | 19.579 | 37.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

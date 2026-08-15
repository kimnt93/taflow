# ExponentiallyWeightedCovariance benchmark (`ewm covariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.21M | 0.006 | 166.87M | 1.266 | 175.01× | 211.31× |
| 10,000 | 0.053 | 187.93M | 0.048 | 209.86M | 12.531 | 235.50× | 262.98× |
| 100,000 | 0.476 | 210.20M | 0.464 | 215.64M | 125.918 | 264.68× | 271.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.105 | 0.70× |
| 1 | 5 | 0.232 | 0.406 | 1.75× |
| 1 | 10 | 0.383 | 0.818 | 2.14× |
| 10 | 1 | 0.048 | 0.101 | 2.11× |
| 10 | 5 | 0.216 | 0.492 | 2.28× |
| 10 | 10 | 0.418 | 0.979 | 2.34× |
| 100 | 1 | 0.044 | 0.204 | 4.67× |
| 100 | 5 | 0.191 | 1.061 | 5.55× |
| 100 | 10 | 0.413 | 2.081 | 5.04× |
| 1,000 | 1 | 0.045 | 1.371 | 30.45× |
| 1,000 | 5 | 0.198 | 6.745 | 34.03× |
| 1,000 | 10 | 0.430 | 13.825 | 32.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

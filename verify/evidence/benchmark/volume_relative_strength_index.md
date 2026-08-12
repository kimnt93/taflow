# VolumeRelativeStrengthIndex benchmark (`VolumeRsi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.25M | 0.012 | 81.55M | 0.228 | 17.63× | 18.61× |
| 10,000 | 0.103 | 97.10M | 0.102 | 98.07M | 0.852 | 8.27× | 8.35× |
| 100,000 | 0.933 | 107.20M | 0.922 | 108.47M | 7.381 | 7.91× | 8.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.278 | 3.84× |
| 1 | 5 | 0.291 | 1.194 | 4.10× |
| 1 | 10 | 0.446 | 2.418 | 5.42× |
| 10 | 1 | 0.053 | 0.224 | 4.24× |
| 10 | 5 | 0.230 | 1.288 | 5.61× |
| 10 | 10 | 0.473 | 2.432 | 5.14× |
| 100 | 1 | 0.051 | 0.237 | 4.70× |
| 100 | 5 | 0.254 | 1.428 | 5.61× |
| 100 | 10 | 0.526 | 2.551 | 4.85× |
| 1,000 | 1 | 0.067 | 0.304 | 4.56× |
| 1,000 | 5 | 0.248 | 1.720 | 6.93× |
| 1,000 | 10 | 0.499 | 3.164 | 6.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.167 | 5.98M | 0.161 | 6.22M | 0.040 | 0.24× | 0.25× |
| 10,000 | 1.488 | 6.72M | 1.478 | 6.77M | 0.156 | 0.10× | 0.11× |
| 100,000 | 15.270 | 6.55M | 15.405 | 6.49M | 1.300 | 0.09× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.148 | 1.41× |
| 1 | 5 | 0.389 | 0.446 | 1.14× |
| 1 | 10 | 0.646 | 0.930 | 1.44× |
| 10 | 1 | 0.068 | 0.092 | 1.35× |
| 10 | 5 | 0.306 | 0.436 | 1.42× |
| 10 | 10 | 0.662 | 0.887 | 1.34× |
| 100 | 1 | 0.085 | 0.093 | 1.10× |
| 100 | 5 | 0.343 | 0.447 | 1.30× |
| 100 | 10 | 0.650 | 0.947 | 1.46× |
| 1,000 | 1 | 0.228 | 0.108 | 0.47× |
| 1,000 | 5 | 0.389 | 0.499 | 1.28× |
| 1,000 | 10 | 0.771 | 1.079 | 1.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

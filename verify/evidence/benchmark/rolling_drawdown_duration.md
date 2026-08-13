# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.84M | 0.022 | 44.68M | 0.120 | 3.70× | 5.36× |
| 10,000 | 0.167 | 59.95M | 0.164 | 60.98M | 0.400 | 2.40× | 2.44× |
| 100,000 | 1.558 | 64.18M | 1.510 | 66.21M | 3.167 | 2.03× | 2.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.197 | 1.76× |
| 1 | 5 | 0.513 | 0.849 | 1.66× |
| 1 | 10 | 0.587 | 1.783 | 3.04× |
| 10 | 1 | 0.066 | 0.159 | 2.42× |
| 10 | 5 | 0.278 | 0.779 | 2.80× |
| 10 | 10 | 0.563 | 1.615 | 2.87× |
| 100 | 1 | 0.068 | 0.165 | 2.42× |
| 100 | 5 | 0.281 | 1.042 | 3.71× |
| 100 | 10 | 0.598 | 1.638 | 2.74× |
| 1,000 | 1 | 0.085 | 0.189 | 2.22× |
| 1,000 | 5 | 0.283 | 1.218 | 4.30× |
| 1,000 | 10 | 0.617 | 1.904 | 3.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

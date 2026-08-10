# GapDown benchmark (`gap down relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.40M | 0.006 | 160.09M | 0.024 | 2.96× | 3.84× |
| 10,000 | 0.033 | 300.16M | 0.030 | 332.82M | 0.043 | 1.30× | 1.44× |
| 100,000 | 0.320 | 312.60M | 0.261 | 382.82M | 0.223 | 0.70× | 0.85× |
| 1,000,000 | 3.164 | 316.04M | 2.726 | 366.83M | 4.330 | 1.37× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.086 | 0.82× |
| 1 | 5 | 0.380 | 0.376 | 0.99× |
| 1 | 10 | 0.473 | 0.769 | 1.62× |
| 10 | 1 | 0.049 | 0.080 | 1.64× |
| 10 | 5 | 0.230 | 0.345 | 1.50× |
| 10 | 10 | 0.479 | 0.748 | 1.56× |
| 100 | 1 | 0.051 | 0.070 | 1.38× |
| 100 | 5 | 0.222 | 0.354 | 1.60× |
| 100 | 10 | 0.507 | 0.758 | 1.49× |
| 1,000 | 1 | 0.050 | 0.076 | 1.54× |
| 1,000 | 5 | 0.232 | 0.485 | 2.09× |
| 1,000 | 10 | 0.509 | 1.157 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

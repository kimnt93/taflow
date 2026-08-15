# MassIndex benchmark (`MassIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.24M | 0.007 | 146.67M | 0.250 | 30.01× | 36.60× |
| 10,000 | 0.059 | 170.11M | 0.053 | 188.85M | 0.829 | 14.10× | 15.66× |
| 100,000 | 0.570 | 175.43M | 0.530 | 188.73M | 7.084 | 12.43× | 13.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.364 | 3.55× |
| 1 | 5 | 0.291 | 1.482 | 5.09× |
| 1 | 10 | 0.437 | 2.798 | 6.40× |
| 10 | 1 | 0.049 | 0.247 | 5.03× |
| 10 | 5 | 0.193 | 1.567 | 8.12× |
| 10 | 10 | 0.422 | 2.623 | 6.21× |
| 100 | 1 | 0.049 | 0.269 | 5.48× |
| 100 | 5 | 0.210 | 1.560 | 7.42× |
| 100 | 10 | 0.406 | 3.012 | 7.42× |
| 1,000 | 1 | 0.056 | 0.319 | 5.75× |
| 1,000 | 5 | 0.204 | 1.909 | 9.37× |
| 1,000 | 10 | 0.454 | 3.306 | 7.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

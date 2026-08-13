# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.98M | 0.023 | 42.73M | 0.030 | 1.12× | 1.29× |
| 10,000 | 0.179 | 55.91M | 0.173 | 57.91M | 0.067 | 0.37× | 0.39× |
| 100,000 | 1.744 | 57.32M | 1.654 | 60.46M | 0.419 | 0.24× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.116 | 1.06× |
| 1 | 5 | 1.173 | 0.491 | 0.42× |
| 1 | 10 | 0.586 | 0.862 | 1.47× |
| 10 | 1 | 0.066 | 0.085 | 1.29× |
| 10 | 5 | 0.276 | 0.400 | 1.45× |
| 10 | 10 | 0.555 | 0.840 | 1.51× |
| 100 | 1 | 0.061 | 0.082 | 1.36× |
| 100 | 5 | 0.277 | 0.415 | 1.49× |
| 100 | 10 | 0.605 | 0.878 | 1.45× |
| 1,000 | 1 | 0.082 | 0.088 | 1.08× |
| 1,000 | 5 | 0.283 | 0.433 | 1.53× |
| 1,000 | 10 | 0.586 | 0.911 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

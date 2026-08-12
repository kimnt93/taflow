# EaseOfMovement benchmark (`EaseOfMovement` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.85M | 0.013 | 76.20M | 0.310 | 13.27× | 23.60× |
| 10,000 | 0.087 | 114.40M | 0.083 | 120.86M | 1.333 | 15.25× | 16.12× |
| 100,000 | 1.264 | 79.13M | 0.772 | 129.53M | 11.483 | 9.09× | 14.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.399 | 4.04× |
| 1 | 5 | 0.356 | 1.608 | 4.51× |
| 1 | 10 | 0.517 | 2.802 | 5.42× |
| 10 | 1 | 0.054 | 0.279 | 5.17× |
| 10 | 5 | 0.301 | 1.669 | 5.54× |
| 10 | 10 | 0.549 | 3.140 | 5.72× |
| 100 | 1 | 0.069 | 0.290 | 4.21× |
| 100 | 5 | 0.268 | 1.664 | 6.21× |
| 100 | 10 | 0.618 | 3.106 | 5.03× |
| 1,000 | 1 | 0.067 | 0.382 | 5.69× |
| 1,000 | 5 | 0.293 | 2.350 | 8.03× |
| 1,000 | 10 | 0.604 | 4.632 | 7.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

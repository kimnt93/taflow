# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 171.03M | 0.005 | 215.91M | 0.048 | 8.25× | 10.41× |
| 10,000 | 0.027 | 366.29M | 0.025 | 404.48M | 0.090 | 3.31× | 3.65× |
| 100,000 | 0.245 | 408.51M | 0.233 | 429.45M | 0.457 | 1.87× | 1.96× |
| 1,000,000 | 2.990 | 334.41M | 2.608 | 383.51M | 4.027 | 1.35× | 1.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.196 | 2.79× |
| 1 | 5 | 0.266 | 0.683 | 2.56× |
| 1 | 10 | 0.516 | 1.312 | 2.54× |
| 10 | 1 | 0.051 | 0.152 | 2.99× |
| 10 | 5 | 0.248 | 0.614 | 2.48× |
| 10 | 10 | 0.480 | 1.328 | 2.76× |
| 100 | 1 | 0.050 | 0.152 | 3.02× |
| 100 | 5 | 0.235 | 0.612 | 2.60× |
| 100 | 10 | 0.483 | 1.266 | 2.62× |
| 1,000 | 1 | 0.054 | 0.164 | 3.06× |
| 1,000 | 5 | 0.249 | 0.604 | 2.43× |
| 1,000 | 10 | 0.514 | 1.214 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.17M | 0.032 | 31.00M | 0.030 | 0.82× | 0.94× |
| 10,000 | 0.242 | 41.26M | 0.232 | 43.11M | 0.040 | 0.17× | 0.17× |
| 100,000 | 2.402 | 41.63M | 2.521 | 39.67M | 0.127 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.106 | 1.08× |
| 1 | 5 | 0.346 | 0.498 | 1.44× |
| 1 | 10 | 0.599 | 1.035 | 1.73× |
| 10 | 1 | 0.083 | 0.096 | 1.14× |
| 10 | 5 | 0.310 | 0.448 | 1.44× |
| 10 | 10 | 0.623 | 0.931 | 1.49× |
| 100 | 1 | 0.067 | 0.089 | 1.33× |
| 100 | 5 | 0.301 | 0.439 | 1.46× |
| 100 | 10 | 0.630 | 0.905 | 1.44× |
| 1,000 | 1 | 0.090 | 0.086 | 0.95× |
| 1,000 | 5 | 0.292 | 0.414 | 1.42× |
| 1,000 | 10 | 0.638 | 0.901 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

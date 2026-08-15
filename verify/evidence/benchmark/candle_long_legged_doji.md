# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.57M | 0.005 | 197.93M | 0.036 | 4.24× | 7.19× |
| 10,000 | 0.069 | 145.02M | 0.064 | 156.98M | 0.095 | 1.37× | 1.49× |
| 100,000 | 0.687 | 145.65M | 0.700 | 142.92M | 0.664 | 0.97× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.120 | 1.00× |
| 1 | 5 | 0.266 | 0.471 | 1.77× |
| 1 | 10 | 0.406 | 0.932 | 2.29× |
| 10 | 1 | 0.042 | 0.092 | 2.18× |
| 10 | 5 | 0.180 | 0.433 | 2.41× |
| 10 | 10 | 0.377 | 0.956 | 2.54× |
| 100 | 1 | 0.060 | 0.098 | 1.63× |
| 100 | 5 | 0.192 | 0.432 | 2.26× |
| 100 | 10 | 0.404 | 0.878 | 2.18× |
| 1,000 | 1 | 0.048 | 0.095 | 1.99× |
| 1,000 | 5 | 0.198 | 0.491 | 2.48× |
| 1,000 | 10 | 0.440 | 0.971 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

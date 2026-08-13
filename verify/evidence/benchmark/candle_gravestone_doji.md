# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.10M | 0.056 | 17.78M | 0.034 | 0.51× | 0.60× |
| 10,000 | 0.472 | 21.17M | 0.470 | 21.26M | 0.096 | 0.20× | 0.20× |
| 100,000 | 4.509 | 22.18M | 4.497 | 22.24M | 0.759 | 0.17× | 0.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.131 | 1.06× |
| 1 | 5 | 0.432 | 0.474 | 1.10× |
| 1 | 10 | 0.647 | 0.921 | 1.42× |
| 10 | 1 | 0.072 | 0.092 | 1.28× |
| 10 | 5 | 0.303 | 0.423 | 1.40× |
| 10 | 10 | 0.636 | 0.874 | 1.37× |
| 100 | 1 | 0.071 | 0.084 | 1.19× |
| 100 | 5 | 0.315 | 0.425 | 1.35× |
| 100 | 10 | 0.677 | 0.886 | 1.31× |
| 1,000 | 1 | 0.120 | 0.100 | 0.83× |
| 1,000 | 5 | 0.328 | 0.476 | 1.45× |
| 1,000 | 10 | 0.649 | 1.006 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

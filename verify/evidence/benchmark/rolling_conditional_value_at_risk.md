# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.130 | 7.67M | 0.125 | 8.00M | 0.321 | 2.46× | 2.56× |
| 10,000 | 1.257 | 7.96M | 1.393 | 7.18M | 1.757 | 1.40× | 1.26× |
| 100,000 | 12.453 | 8.03M | 12.444 | 8.04M | 15.120 | 1.21× | 1.22× |
| 1,000,000 | 125.605 | 7.96M | 126.375 | 7.91M | 158.277 | 1.26× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.595 | 5.31× |
| 1 | 5 | 0.270 | 1.203 | 4.45× |
| 1 | 10 | 0.507 | 2.689 | 5.30× |
| 10 | 1 | 0.049 | 0.240 | 4.91× |
| 10 | 5 | 0.243 | 1.165 | 4.80× |
| 10 | 10 | 0.487 | 2.934 | 6.02× |
| 100 | 1 | 0.105 | 0.267 | 2.53× |
| 100 | 5 | 0.231 | 1.393 | 6.03× |
| 100 | 10 | 0.479 | 2.724 | 5.68× |
| 1,000 | 1 | 0.190 | 0.410 | 2.15× |
| 1,000 | 5 | 0.326 | 2.217 | 6.79× |
| 1,000 | 10 | 0.599 | 4.262 | 7.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

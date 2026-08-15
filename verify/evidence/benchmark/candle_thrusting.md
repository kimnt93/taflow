# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 153.87M | 0.003 | 341.40M | 0.033 | 5.07× | 11.25× |
| 10,000 | 0.066 | 150.56M | 0.063 | 159.04M | 0.120 | 1.80× | 1.90× |
| 100,000 | 0.835 | 119.81M | 0.822 | 121.65M | 0.914 | 1.10× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.169 | 2.31× |
| 1 | 5 | 0.302 | 0.456 | 1.51× |
| 1 | 10 | 0.397 | 0.932 | 2.35× |
| 10 | 1 | 0.040 | 0.086 | 2.14× |
| 10 | 5 | 0.203 | 0.480 | 2.36× |
| 10 | 10 | 0.415 | 0.943 | 2.27× |
| 100 | 1 | 0.047 | 0.093 | 2.00× |
| 100 | 5 | 0.205 | 0.426 | 2.07× |
| 100 | 10 | 0.399 | 0.955 | 2.39× |
| 1,000 | 1 | 0.052 | 0.096 | 1.86× |
| 1,000 | 5 | 0.203 | 0.498 | 2.45× |
| 1,000 | 10 | 0.428 | 1.050 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

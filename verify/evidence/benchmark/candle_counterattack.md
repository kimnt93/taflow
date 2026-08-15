# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.87M | 0.011 | 88.96M | 0.034 | 2.46× | 3.05× |
| 10,000 | 0.160 | 62.43M | 0.155 | 64.45M | 0.133 | 0.83× | 0.86× |
| 100,000 | 1.662 | 60.16M | 1.611 | 62.07M | 1.101 | 0.66× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.123 | 1.21× |
| 1 | 5 | 0.276 | 0.486 | 1.76× |
| 1 | 10 | 0.402 | 0.887 | 2.21× |
| 10 | 1 | 0.049 | 0.098 | 1.99× |
| 10 | 5 | 0.216 | 0.469 | 2.17× |
| 10 | 10 | 0.398 | 0.909 | 2.28× |
| 100 | 1 | 0.044 | 0.094 | 2.16× |
| 100 | 5 | 0.187 | 0.431 | 2.30× |
| 100 | 10 | 0.422 | 0.918 | 2.17× |
| 1,000 | 1 | 0.058 | 0.102 | 1.78× |
| 1,000 | 5 | 0.205 | 0.483 | 2.35× |
| 1,000 | 10 | 0.425 | 1.064 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

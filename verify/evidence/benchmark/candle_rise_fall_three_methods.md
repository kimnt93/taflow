# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.187 | 5.35M | 0.177 | 5.65M | 0.035 | 0.19× | 0.20× |
| 10,000 | 1.710 | 5.85M | 1.679 | 5.95M | 0.113 | 0.07× | 0.07× |
| 100,000 | 16.633 | 6.01M | 16.767 | 5.96M | 0.902 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.136 | 1.25× |
| 1 | 5 | 0.437 | 0.485 | 1.11× |
| 1 | 10 | 0.641 | 0.933 | 1.45× |
| 10 | 1 | 0.072 | 0.093 | 1.28× |
| 10 | 5 | 0.307 | 0.426 | 1.39× |
| 10 | 10 | 0.646 | 0.892 | 1.38× |
| 100 | 1 | 0.084 | 0.092 | 1.09× |
| 100 | 5 | 0.324 | 0.466 | 1.44× |
| 100 | 10 | 0.675 | 1.018 | 1.51× |
| 1,000 | 1 | 0.261 | 0.105 | 0.40× |
| 1,000 | 5 | 0.533 | 0.504 | 0.95× |
| 1,000 | 10 | 0.804 | 1.022 | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

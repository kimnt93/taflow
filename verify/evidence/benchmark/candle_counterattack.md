# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.29M | 0.004 | 246.76M | 0.040 | 5.80× | 9.98× |
| 10,000 | 0.066 | 151.88M | 0.059 | 170.65M | 0.146 | 2.22× | 2.49× |
| 100,000 | 0.893 | 112.01M | 0.846 | 118.16M | 1.164 | 1.30× | 1.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.130 | 2.03× |
| 1 | 5 | 0.239 | 0.442 | 1.85× |
| 1 | 10 | 0.379 | 1.024 | 2.70× |
| 10 | 1 | 0.057 | 0.118 | 2.09× |
| 10 | 5 | 0.257 | 0.510 | 1.99× |
| 10 | 10 | 0.373 | 0.899 | 2.41× |
| 100 | 1 | 0.050 | 0.097 | 1.95× |
| 100 | 5 | 0.193 | 0.517 | 2.68× |
| 100 | 10 | 0.414 | 0.917 | 2.22× |
| 1,000 | 1 | 0.054 | 0.102 | 1.89× |
| 1,000 | 5 | 0.184 | 0.490 | 2.66× |
| 1,000 | 10 | 0.426 | 1.035 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

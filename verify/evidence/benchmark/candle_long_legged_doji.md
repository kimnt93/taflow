# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.02M | 0.013 | 74.33M | 0.042 | 2.33× | 3.09× |
| 10,000 | 0.109 | 92.02M | 0.097 | 102.77M | 0.128 | 1.18× | 1.32× |
| 100,000 | 0.993 | 100.72M | 0.948 | 105.46M | 0.819 | 0.82× | 0.86× |
| 1,000,000 | 9.072 | 110.23M | 9.313 | 107.38M | 9.498 | 1.05× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | 0.134 | 0.90× |
| 1 | 5 | 0.304 | 0.545 | 1.79× |
| 1 | 10 | 0.589 | 1.213 | 2.06× |
| 10 | 1 | 0.076 | 0.107 | 1.40× |
| 10 | 5 | 0.365 | 0.546 | 1.50× |
| 10 | 10 | 0.635 | 1.157 | 1.82× |
| 100 | 1 | 0.074 | 0.124 | 1.68× |
| 100 | 5 | 0.372 | 0.524 | 1.41× |
| 100 | 10 | 0.634 | 1.210 | 1.91× |
| 1,000 | 1 | 0.077 | 0.146 | 1.90× |
| 1,000 | 5 | 0.416 | 0.589 | 1.41× |
| 1,000 | 10 | 0.665 | 1.332 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

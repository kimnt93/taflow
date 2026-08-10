# PreviousHighLow benchmark (`previous-session high-low` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.97M | 0.014 | 71.16M | 0.608 | 35.86× | 43.28× |
| 10,000 | 0.104 | 95.85M | 0.094 | 106.58M | 5.725 | 54.87× | 61.02× |
| 100,000 | 0.961 | 104.02M | 0.937 | 106.67M | 57.789 | 60.11× | 61.64× |
| 1,000,000 | 10.769 | 92.86M | 8.905 | 112.29M | 601.663 | 55.87× | 67.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.091 | 0.95× |
| 1 | 5 | 0.283 | 0.336 | 1.19× |
| 1 | 10 | 0.472 | 0.668 | 1.41× |
| 10 | 1 | 0.054 | 0.074 | 1.35× |
| 10 | 5 | 0.227 | 0.344 | 1.51× |
| 10 | 10 | 0.480 | 0.717 | 1.49× |
| 100 | 1 | 0.052 | 0.122 | 2.36× |
| 100 | 5 | 0.247 | 0.611 | 2.47× |
| 100 | 10 | 0.521 | 1.246 | 2.39× |
| 1,000 | 1 | 0.062 | 0.673 | 10.80× |
| 1,000 | 5 | 0.255 | 3.291 | 12.91× |
| 1,000 | 10 | 0.526 | 7.288 | 13.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

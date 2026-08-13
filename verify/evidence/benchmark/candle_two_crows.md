# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.59M | 0.053 | 18.71M | 0.033 | 0.51× | 0.61× |
| 10,000 | 0.411 | 24.34M | 0.399 | 25.06M | 0.104 | 0.25× | 0.26× |
| 100,000 | 3.878 | 25.79M | 4.314 | 23.18M | 0.857 | 0.22× | 0.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.119 | 1.18× |
| 1 | 5 | 0.372 | 0.489 | 1.31× |
| 1 | 10 | 0.653 | 0.918 | 1.41× |
| 10 | 1 | 0.071 | 0.092 | 1.29× |
| 10 | 5 | 0.317 | 0.425 | 1.34× |
| 10 | 10 | 0.680 | 0.881 | 1.29× |
| 100 | 1 | 0.079 | 0.088 | 1.12× |
| 100 | 5 | 0.315 | 0.416 | 1.32× |
| 100 | 10 | 0.675 | 0.897 | 1.33× |
| 1,000 | 1 | 0.119 | 0.101 | 0.85× |
| 1,000 | 5 | 0.332 | 0.470 | 1.42× |
| 1,000 | 10 | 0.700 | 0.988 | 1.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

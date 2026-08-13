# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.110 | 9.09M | 0.097 | 10.29M | 0.036 | 0.33× | 0.37× |
| 10,000 | 0.892 | 11.21M | 0.887 | 11.27M | 0.134 | 0.15× | 0.15× |
| 100,000 | 9.044 | 11.06M | 8.487 | 11.78M | 1.070 | 0.12× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.128 | 1.00× |
| 1 | 5 | 0.365 | 0.474 | 1.30× |
| 1 | 10 | 0.636 | 0.884 | 1.39× |
| 10 | 1 | 0.066 | 0.089 | 1.35× |
| 10 | 5 | 0.316 | 0.425 | 1.34× |
| 10 | 10 | 0.639 | 0.906 | 1.42× |
| 100 | 1 | 0.093 | 0.096 | 1.03× |
| 100 | 5 | 0.321 | 0.454 | 1.41× |
| 100 | 10 | 0.677 | 0.924 | 1.36× |
| 1,000 | 1 | 0.159 | 0.105 | 0.66× |
| 1,000 | 5 | 0.405 | 0.725 | 1.79× |
| 1,000 | 10 | 0.817 | 1.067 | 1.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

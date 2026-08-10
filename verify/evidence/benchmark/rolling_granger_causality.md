# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.901 | 526.17K | 1.868 | 535.29K | 8.348 | 4.39× | 4.47× |
| 10,000 | 20.417 | 489.78K | 20.148 | 496.33K | 85.812 | 4.20× | 4.26× |
| 100,000 | 209.896 | 476.43K | 191.823 | 521.31K | 853.189 | 4.06× | 4.45× |
| 1,000,000 | 1921.262 | 520.49K | 1992.055 | 501.99K | 8460.189 | 4.40× | 4.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.486 | 4.65× |
| 1 | 5 | 0.269 | 1.333 | 4.95× |
| 1 | 10 | 0.516 | 2.710 | 5.25× |
| 10 | 1 | 0.054 | 0.260 | 4.80× |
| 10 | 5 | 0.270 | 1.436 | 5.31× |
| 10 | 10 | 0.511 | 3.196 | 6.25× |
| 100 | 1 | 0.158 | 0.692 | 4.38× |
| 100 | 5 | 0.333 | 3.900 | 11.70× |
| 100 | 10 | 0.599 | 6.774 | 11.30× |
| 1,000 | 1 | 1.927 | 8.203 | 4.26× |
| 1,000 | 5 | 3.096 | 41.707 | 13.47× |
| 1,000 | 10 | 4.021 | 86.103 | 21.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

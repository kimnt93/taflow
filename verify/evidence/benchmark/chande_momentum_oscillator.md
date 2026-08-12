# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.43M | 0.008 | 132.30M | 0.039 | 4.43× | 5.12× |
| 10,000 | 0.060 | 166.24M | 0.056 | 177.37M | 0.091 | 1.51× | 1.61× |
| 100,000 | 0.555 | 180.24M | 0.549 | 182.28M | 0.609 | 1.10× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.129 | 1.62× |
| 1 | 5 | 0.328 | 0.522 | 1.59× |
| 1 | 10 | 0.552 | 1.010 | 1.83× |
| 10 | 1 | 0.051 | 0.097 | 1.91× |
| 10 | 5 | 0.213 | 0.460 | 2.16× |
| 10 | 10 | 0.472 | 0.949 | 2.01× |
| 100 | 1 | 0.047 | 0.097 | 2.08× |
| 100 | 5 | 0.253 | 0.483 | 1.91× |
| 100 | 10 | 0.479 | 0.965 | 2.02× |
| 1,000 | 1 | 0.054 | 0.100 | 1.86× |
| 1,000 | 5 | 0.240 | 0.517 | 2.16× |
| 1,000 | 10 | 0.499 | 1.004 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

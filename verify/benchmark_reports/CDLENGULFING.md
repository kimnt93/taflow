# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.61M | 0.006 | 160.61M | 0.032 | 4.05× | 5.10× |
| 10,000 | 0.059 | 168.92M | 0.056 | 177.98M | 0.085 | 1.44× | 1.51× |
| 100,000 | 0.671 | 148.93M | 0.636 | 157.22M | 0.545 | 0.81× | 0.86× |
| 1,000,000 | 6.551 | 152.65M | 6.551 | 152.64M | 5.435 | 0.83× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.102 | 1.11× |
| 1 | 5 | 0.379 | 0.512 | 1.35× |
| 1 | 10 | 0.482 | 0.923 | 1.91× |
| 10 | 1 | 0.052 | 0.090 | 1.74× |
| 10 | 5 | 0.237 | 0.433 | 1.83× |
| 10 | 10 | 0.498 | 0.932 | 1.87× |
| 100 | 1 | 0.057 | 0.093 | 1.64× |
| 100 | 5 | 0.263 | 0.442 | 1.68× |
| 100 | 10 | 0.568 | 1.045 | 1.84× |
| 1,000 | 1 | 0.083 | 0.152 | 1.84× |
| 1,000 | 5 | 0.317 | 0.685 | 2.16× |
| 1,000 | 10 | 0.596 | 1.029 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.82M | 0.011 | 92.21M | 0.034 | 2.43× | 3.16× |
| 10,000 | 0.078 | 128.20M | 0.072 | 137.97M | 0.100 | 1.28× | 1.38× |
| 100,000 | 0.734 | 136.17M | 0.733 | 136.49M | 0.681 | 0.93× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.102 | 1.19× |
| 1 | 5 | 0.280 | 0.474 | 1.70× |
| 1 | 10 | 0.621 | 0.975 | 1.57× |
| 10 | 1 | 0.056 | 0.090 | 1.61× |
| 10 | 5 | 0.275 | 0.444 | 1.62× |
| 10 | 10 | 0.573 | 1.002 | 1.75× |
| 100 | 1 | 0.062 | 0.087 | 1.41× |
| 100 | 5 | 0.278 | 0.463 | 1.66× |
| 100 | 10 | 0.578 | 1.022 | 1.77× |
| 1,000 | 1 | 0.076 | 0.113 | 1.48× |
| 1,000 | 5 | 0.280 | 0.522 | 1.86× |
| 1,000 | 10 | 0.698 | 1.166 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

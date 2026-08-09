# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.66M | 0.009 | 115.22M | 0.039 | 3.79× | 4.52× |
| 10,000 | 0.072 | 138.38M | 0.070 | 143.48M | 0.123 | 1.70× | 1.77× |
| 100,000 | 0.877 | 114.07M | 0.750 | 133.35M | 0.984 | 1.12× | 1.31× |
| 1,000,000 | 7.498 | 133.37M | 7.418 | 134.81M | 9.449 | 1.26× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.104 | 1.35× |
| 1 | 5 | 0.253 | 0.490 | 1.93× |
| 1 | 10 | 0.525 | 0.971 | 1.85× |
| 10 | 1 | 0.051 | 0.089 | 1.74× |
| 10 | 5 | 0.250 | 0.447 | 1.79× |
| 10 | 10 | 0.522 | 0.928 | 1.78× |
| 100 | 1 | 0.055 | 0.095 | 1.72× |
| 100 | 5 | 0.287 | 0.475 | 1.66× |
| 100 | 10 | 0.535 | 0.958 | 1.79× |
| 1,000 | 1 | 0.064 | 0.103 | 1.60× |
| 1,000 | 5 | 0.258 | 0.496 | 1.92× |
| 1,000 | 10 | 0.568 | 1.072 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.51M | 0.010 | 99.44M | 0.032 | 2.31× | 3.21× |
| 10,000 | 0.080 | 124.38M | 0.074 | 135.34M | 0.088 | 1.09× | 1.19× |
| 100,000 | 0.853 | 117.18M | 0.819 | 122.17M | 0.640 | 0.75× | 0.78× |
| 1,000,000 | 8.702 | 114.92M | 8.174 | 122.34M | 6.257 | 0.72× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.119 | 1.79× |
| 1 | 5 | 0.317 | 0.477 | 1.50× |
| 1 | 10 | 0.577 | 0.991 | 1.72× |
| 10 | 1 | 0.059 | 0.085 | 1.46× |
| 10 | 5 | 0.248 | 0.429 | 1.73× |
| 10 | 10 | 0.522 | 0.948 | 1.82× |
| 100 | 1 | 0.061 | 0.096 | 1.57× |
| 100 | 5 | 0.278 | 0.466 | 1.67× |
| 100 | 10 | 0.559 | 1.005 | 1.80× |
| 1,000 | 1 | 0.064 | 0.097 | 1.50× |
| 1,000 | 5 | 0.292 | 0.510 | 1.75× |
| 1,000 | 10 | 0.641 | 1.077 | 1.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

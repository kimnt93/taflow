# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.61M | 0.008 | 118.61M | 0.032 | 2.55× | 3.75× |
| 10,000 | 0.051 | 196.75M | 0.049 | 202.96M | 0.094 | 1.86× | 1.92× |
| 100,000 | 0.587 | 170.46M | 0.590 | 169.46M | 0.635 | 1.08× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.114 | 0.97× |
| 1 | 5 | 0.262 | 0.468 | 1.79× |
| 1 | 10 | 0.515 | 0.909 | 1.76× |
| 10 | 1 | 0.054 | 0.093 | 1.71× |
| 10 | 5 | 0.301 | 0.505 | 1.68× |
| 10 | 10 | 0.537 | 0.891 | 1.66× |
| 100 | 1 | 0.056 | 0.090 | 1.60× |
| 100 | 5 | 0.270 | 0.438 | 1.63× |
| 100 | 10 | 0.619 | 0.935 | 1.51× |
| 1,000 | 1 | 0.086 | 0.096 | 1.12× |
| 1,000 | 5 | 0.259 | 0.478 | 1.84× |
| 1,000 | 10 | 0.550 | 1.107 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

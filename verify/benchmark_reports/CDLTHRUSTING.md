# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.56M | 0.008 | 128.09M | 0.035 | 3.71× | 4.45× |
| 10,000 | 0.069 | 144.68M | 0.066 | 151.23M | 0.119 | 1.73× | 1.80× |
| 100,000 | 0.869 | 115.09M | 0.851 | 117.49M | 0.974 | 1.12× | 1.14× |
| 1,000,000 | 9.177 | 108.97M | 8.765 | 114.08M | 9.098 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.109 | 1.11× |
| 1 | 5 | 0.396 | 0.578 | 1.46× |
| 1 | 10 | 0.500 | 0.928 | 1.86× |
| 10 | 1 | 0.054 | 0.094 | 1.73× |
| 10 | 5 | 0.234 | 0.446 | 1.91× |
| 10 | 10 | 0.508 | 0.930 | 1.83× |
| 100 | 1 | 0.056 | 0.093 | 1.67× |
| 100 | 5 | 0.258 | 0.447 | 1.73× |
| 100 | 10 | 0.510 | 0.954 | 1.87× |
| 1,000 | 1 | 0.066 | 0.108 | 1.65× |
| 1,000 | 5 | 0.254 | 0.491 | 1.93× |
| 1,000 | 10 | 0.542 | 1.031 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

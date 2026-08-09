# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.35M | 0.007 | 141.24M | 0.034 | 3.81× | 4.79× |
| 10,000 | 0.049 | 206.03M | 0.044 | 228.21M | 0.090 | 1.85× | 2.05× |
| 100,000 | 0.554 | 180.45M | 0.545 | 183.49M | 0.583 | 1.05× | 1.07× |
| 1,000,000 | 5.953 | 167.98M | 5.845 | 171.10M | 5.991 | 1.01× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.116 | 1.05× |
| 1 | 5 | 0.297 | 0.552 | 1.86× |
| 1 | 10 | 0.789 | 1.135 | 1.44× |
| 10 | 1 | 0.055 | 0.101 | 1.83× |
| 10 | 5 | 0.250 | 0.476 | 1.90× |
| 10 | 10 | 0.520 | 0.954 | 1.83× |
| 100 | 1 | 0.053 | 0.093 | 1.76× |
| 100 | 5 | 0.249 | 0.444 | 1.78× |
| 100 | 10 | 0.521 | 0.921 | 1.77× |
| 1,000 | 1 | 0.063 | 0.105 | 1.67× |
| 1,000 | 5 | 0.254 | 0.482 | 1.89× |
| 1,000 | 10 | 0.500 | 0.995 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

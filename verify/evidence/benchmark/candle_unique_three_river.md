# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.98M | 0.010 | 99.83M | 0.034 | 2.48× | 3.34× |
| 10,000 | 0.129 | 77.73M | 0.122 | 81.76M | 0.080 | 0.62× | 0.65× |
| 100,000 | 1.295 | 77.21M | 1.286 | 77.78M | 0.588 | 0.45× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.115 | 0.85× |
| 1 | 5 | 0.310 | 0.441 | 1.42× |
| 1 | 10 | 0.394 | 0.915 | 2.32× |
| 10 | 1 | 0.045 | 0.084 | 1.89× |
| 10 | 5 | 0.229 | 0.466 | 2.04× |
| 10 | 10 | 0.392 | 0.937 | 2.39× |
| 100 | 1 | 0.042 | 0.098 | 2.35× |
| 100 | 5 | 0.189 | 0.421 | 2.23× |
| 100 | 10 | 0.431 | 1.035 | 2.40× |
| 1,000 | 1 | 0.059 | 0.100 | 1.71× |
| 1,000 | 5 | 0.212 | 0.481 | 2.27× |
| 1,000 | 10 | 0.443 | 1.049 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

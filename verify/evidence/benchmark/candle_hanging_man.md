# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.13M | 0.017 | 57.41M | 0.043 | 2.18× | 2.45× |
| 10,000 | 0.171 | 58.60M | 0.195 | 51.23M | 0.166 | 0.97× | 0.85× |
| 100,000 | 1.577 | 63.41M | 1.821 | 54.91M | 1.371 | 0.87× | 0.75× |
| 1,000,000 | 17.127 | 58.39M | 18.626 | 53.69M | 14.272 | 0.83× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.106 | 1.21× |
| 1 | 5 | 0.329 | 0.463 | 1.41× |
| 1 | 10 | 0.533 | 0.898 | 1.68× |
| 10 | 1 | 0.053 | 0.091 | 1.72× |
| 10 | 5 | 0.237 | 0.419 | 1.77× |
| 10 | 10 | 0.531 | 0.897 | 1.69× |
| 100 | 1 | 0.057 | 0.086 | 1.51× |
| 100 | 5 | 0.241 | 0.432 | 1.79× |
| 100 | 10 | 0.543 | 0.897 | 1.65× |
| 1,000 | 1 | 0.072 | 0.105 | 1.45× |
| 1,000 | 5 | 0.288 | 0.502 | 1.74× |
| 1,000 | 10 | 0.559 | 1.056 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

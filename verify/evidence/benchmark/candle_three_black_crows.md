# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.88M | 0.004 | 275.85M | 0.029 | 4.33× | 8.07× |
| 10,000 | 0.058 | 171.35M | 0.052 | 192.37M | 0.082 | 1.40× | 1.57× |
| 100,000 | 0.705 | 141.92M | 0.704 | 142.12M | 0.583 | 0.83× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.193 | 1.06× |
| 1 | 5 | 0.242 | 0.456 | 1.88× |
| 1 | 10 | 0.394 | 0.871 | 2.21× |
| 10 | 1 | 0.044 | 0.093 | 2.12× |
| 10 | 5 | 0.192 | 0.410 | 2.13× |
| 10 | 10 | 0.382 | 0.858 | 2.24× |
| 100 | 1 | 0.043 | 0.089 | 2.08× |
| 100 | 5 | 0.176 | 0.397 | 2.25× |
| 100 | 10 | 0.376 | 0.862 | 2.29× |
| 1,000 | 1 | 0.054 | 0.091 | 1.69× |
| 1,000 | 5 | 0.185 | 0.441 | 2.39× |
| 1,000 | 10 | 0.433 | 0.920 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

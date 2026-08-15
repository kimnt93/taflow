# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.55M | 0.004 | 276.68M | 0.038 | 5.50× | 10.39× |
| 10,000 | 0.091 | 109.41M | 0.085 | 116.97M | 0.131 | 1.43× | 1.53× |
| 100,000 | 1.026 | 97.42M | 1.051 | 95.11M | 1.111 | 1.08× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.106 | 1.48× |
| 1 | 5 | 0.225 | 0.469 | 2.08× |
| 1 | 10 | 0.393 | 1.101 | 2.81× |
| 10 | 1 | 0.045 | 0.089 | 1.97× |
| 10 | 5 | 0.193 | 0.463 | 2.40× |
| 10 | 10 | 0.398 | 0.932 | 2.34× |
| 100 | 1 | 0.047 | 0.098 | 2.09× |
| 100 | 5 | 0.220 | 0.535 | 2.43× |
| 100 | 10 | 0.425 | 0.947 | 2.23× |
| 1,000 | 1 | 0.059 | 0.103 | 1.76× |
| 1,000 | 5 | 0.216 | 0.534 | 2.47× |
| 1,000 | 10 | 0.525 | 1.143 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

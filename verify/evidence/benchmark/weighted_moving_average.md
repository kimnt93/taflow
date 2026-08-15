# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 247.78M | 0.003 | 321.72M | 0.035 | 8.79× | 11.41× |
| 10,000 | 0.032 | 312.43M | 0.023 | 434.06M | 0.051 | 1.58× | 2.20× |
| 100,000 | 0.236 | 423.74M | 0.211 | 474.75M | 0.241 | 1.02× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.111 | 0.96× |
| 1 | 5 | 0.215 | 0.469 | 2.19× |
| 1 | 10 | 0.414 | 0.999 | 2.41× |
| 10 | 1 | 0.043 | 0.090 | 2.08× |
| 10 | 5 | 0.215 | 0.493 | 2.29× |
| 10 | 10 | 0.402 | 0.980 | 2.44× |
| 100 | 1 | 0.046 | 0.090 | 1.93× |
| 100 | 5 | 0.196 | 0.477 | 2.43× |
| 100 | 10 | 0.489 | 1.015 | 2.08× |
| 1,000 | 1 | 0.047 | 0.092 | 1.97× |
| 1,000 | 5 | 0.196 | 0.452 | 2.31× |
| 1,000 | 10 | 0.417 | 1.011 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

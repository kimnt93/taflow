# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.283 | 3.53M | 0.270 | 3.70M | 0.252 | 0.89× | 0.93× |
| 10,000 | 2.645 | 3.78M | 2.887 | 3.46M | 1.205 | 0.46× | 0.42× |
| 100,000 | 27.059 | 3.70M | 26.887 | 3.72M | 11.139 | 0.41× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.223 | 2.09× |
| 1 | 5 | 0.513 | 0.991 | 1.93× |
| 1 | 10 | 0.592 | 2.057 | 3.47× |
| 10 | 1 | 0.079 | 0.183 | 2.32× |
| 10 | 5 | 0.312 | 0.915 | 2.93× |
| 10 | 10 | 0.619 | 2.098 | 3.39× |
| 100 | 1 | 0.096 | 0.192 | 2.00× |
| 100 | 5 | 0.299 | 1.016 | 3.40× |
| 100 | 10 | 0.641 | 2.166 | 3.38× |
| 1,000 | 1 | 0.352 | 0.317 | 0.90× |
| 1,000 | 5 | 0.633 | 1.553 | 2.45× |
| 1,000 | 10 | 0.941 | 3.312 | 3.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

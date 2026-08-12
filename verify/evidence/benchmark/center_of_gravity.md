# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.53M | 0.021 | 48.76M | 0.174 | 8.63× | 8.50× |
| 10,000 | 0.188 | 53.07M | 0.187 | 53.52M | 0.615 | 3.26× | 3.29× |
| 100,000 | 2.203 | 45.38M | 1.921 | 52.04M | 5.471 | 2.48× | 2.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.221 | 2.36× |
| 1 | 5 | 0.253 | 0.980 | 3.88× |
| 1 | 10 | 0.468 | 2.093 | 4.48× |
| 10 | 1 | 0.048 | 0.195 | 4.04× |
| 10 | 5 | 0.255 | 0.981 | 3.85× |
| 10 | 10 | 0.488 | 2.138 | 4.38× |
| 100 | 1 | 0.053 | 0.207 | 3.90× |
| 100 | 5 | 0.253 | 0.999 | 3.95× |
| 100 | 10 | 0.501 | 2.221 | 4.43× |
| 1,000 | 1 | 0.070 | 0.254 | 3.62× |
| 1,000 | 5 | 0.257 | 1.232 | 4.79× |
| 1,000 | 10 | 0.510 | 2.715 | 5.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.29M | 0.048 | 20.81M | 0.047 | 0.81× | 0.97× |
| 10,000 | 0.420 | 23.80M | 0.437 | 22.87M | 0.093 | 0.22× | 0.21× |
| 100,000 | 4.000 | 25.00M | 3.719 | 26.89M | 0.938 | 0.23× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.105 | 0.91× |
| 1 | 5 | 0.411 | 0.461 | 1.12× |
| 1 | 10 | 0.647 | 0.929 | 1.44× |
| 10 | 1 | 0.064 | 0.092 | 1.43× |
| 10 | 5 | 0.310 | 0.451 | 1.45× |
| 10 | 10 | 0.613 | 0.895 | 1.46× |
| 100 | 1 | 0.070 | 0.093 | 1.33× |
| 100 | 5 | 0.284 | 0.434 | 1.53× |
| 100 | 10 | 0.651 | 1.020 | 1.57× |
| 1,000 | 1 | 0.121 | 0.097 | 0.81× |
| 1,000 | 5 | 0.327 | 0.511 | 1.56× |
| 1,000 | 10 | 0.841 | 1.146 | 1.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

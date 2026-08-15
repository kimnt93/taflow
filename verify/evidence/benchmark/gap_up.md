# GapUp benchmark (`gap up relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.06M | 0.003 | 286.88M | 0.022 | 4.62× | 6.43× |
| 10,000 | 0.032 | 315.97M | 0.027 | 363.71M | 0.042 | 1.33× | 1.53× |
| 100,000 | 0.275 | 363.10M | 0.254 | 394.35M | 0.275 | 1.00× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.156 | 0.134 | 0.86× |
| 1 | 5 | 0.304 | 0.391 | 1.29× |
| 1 | 10 | 0.400 | 0.730 | 1.83× |
| 10 | 1 | 0.042 | 0.072 | 1.71× |
| 10 | 5 | 0.187 | 0.358 | 1.91× |
| 10 | 10 | 0.390 | 0.770 | 1.97× |
| 100 | 1 | 0.045 | 0.071 | 1.59× |
| 100 | 5 | 0.192 | 0.365 | 1.90× |
| 100 | 10 | 0.402 | 0.781 | 1.94× |
| 1,000 | 1 | 0.054 | 0.083 | 1.53× |
| 1,000 | 5 | 0.197 | 0.488 | 2.48× |
| 1,000 | 10 | 0.430 | 1.119 | 2.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.54M | 0.009 | 105.64M | 0.041 | 4.36× | 4.29× |
| 10,000 | 0.060 | 165.88M | 0.054 | 184.07M | 0.081 | 1.35× | 1.49× |
| 100,000 | 0.530 | 188.74M | 0.511 | 195.87M | 0.508 | 0.96× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.109 | 1.70× |
| 1 | 5 | 0.377 | 0.502 | 1.33× |
| 1 | 10 | 0.503 | 0.940 | 1.87× |
| 10 | 1 | 0.047 | 0.096 | 2.03× |
| 10 | 5 | 0.214 | 0.445 | 2.08× |
| 10 | 10 | 0.463 | 1.001 | 2.16× |
| 100 | 1 | 0.051 | 0.094 | 1.85× |
| 100 | 5 | 0.231 | 0.465 | 2.01× |
| 100 | 10 | 0.473 | 0.962 | 2.03× |
| 1,000 | 1 | 0.062 | 0.096 | 1.54× |
| 1,000 | 5 | 0.247 | 0.471 | 1.90× |
| 1,000 | 10 | 0.514 | 0.973 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

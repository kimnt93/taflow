# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.60M | 0.008 | 121.63M | 0.008 | 0.78× | 0.93× |
| 10,000 | 0.059 | 169.43M | 0.056 | 179.92M | 0.048 | 0.81× | 0.86× |
| 100,000 | 0.570 | 175.39M | 0.541 | 184.92M | 0.440 | 0.77× | 0.81× |
| 1,000,000 | 6.416 | 155.85M | 6.212 | 160.98M | 4.465 | 0.70× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.079 | 0.62× |
| 1 | 5 | 0.256 | 0.215 | 0.84× |
| 1 | 10 | 0.449 | 0.409 | 0.91× |
| 10 | 1 | 0.049 | 0.045 | 0.93× |
| 10 | 5 | 0.223 | 0.203 | 0.91× |
| 10 | 10 | 0.530 | 0.459 | 0.87× |
| 100 | 1 | 0.049 | 0.046 | 0.95× |
| 100 | 5 | 0.249 | 0.202 | 0.81× |
| 100 | 10 | 0.511 | 0.444 | 0.87× |
| 1,000 | 1 | 0.060 | 0.055 | 0.92× |
| 1,000 | 5 | 0.248 | 0.224 | 0.90× |
| 1,000 | 10 | 0.495 | 0.503 | 1.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

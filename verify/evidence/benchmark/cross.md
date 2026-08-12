# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.16M | 0.008 | 122.39M | 0.021 | 2.18× | 2.56× |
| 10,000 | 0.058 | 172.40M | 0.051 | 194.94M | 0.047 | 0.81× | 0.92× |
| 100,000 | 0.487 | 205.52M | 0.607 | 164.87M | 0.314 | 0.64× | 0.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.077 | 0.70× |
| 1 | 5 | 0.431 | 0.367 | 0.85× |
| 1 | 10 | 0.482 | 0.705 | 1.46× |
| 10 | 1 | 0.048 | 0.068 | 1.43× |
| 10 | 5 | 0.251 | 0.337 | 1.34× |
| 10 | 10 | 0.450 | 0.726 | 1.61× |
| 100 | 1 | 0.051 | 0.072 | 1.41× |
| 100 | 5 | 0.221 | 0.329 | 1.49× |
| 100 | 10 | 0.491 | 0.751 | 1.53× |
| 1,000 | 1 | 0.056 | 0.078 | 1.38× |
| 1,000 | 5 | 0.232 | 0.664 | 2.86× |
| 1,000 | 10 | 0.513 | 1.196 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

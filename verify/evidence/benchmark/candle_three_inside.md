# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.93M | 0.009 | 108.96M | 0.037 | 3.12× | 4.00× |
| 10,000 | 0.098 | 101.82M | 0.088 | 113.26M | 0.134 | 1.36× | 1.51× |
| 100,000 | 1.002 | 99.75M | 1.018 | 98.24M | 1.083 | 1.08× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.104 | 1.53× |
| 1 | 5 | 0.269 | 0.444 | 1.65× |
| 1 | 10 | 0.525 | 0.899 | 1.71× |
| 10 | 1 | 0.052 | 0.085 | 1.64× |
| 10 | 5 | 0.252 | 0.413 | 1.64× |
| 10 | 10 | 0.531 | 0.909 | 1.71× |
| 100 | 1 | 0.056 | 0.093 | 1.64× |
| 100 | 5 | 0.267 | 0.432 | 1.62× |
| 100 | 10 | 0.554 | 0.935 | 1.69× |
| 1,000 | 1 | 0.068 | 0.099 | 1.46× |
| 1,000 | 5 | 0.250 | 0.489 | 1.95× |
| 1,000 | 10 | 0.559 | 0.984 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

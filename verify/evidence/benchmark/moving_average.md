# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.42M | 0.008 | 123.16M | 0.059 | 6.79× | 7.31× |
| 10,000 | 0.032 | 314.59M | 0.024 | 408.55M | 0.071 | 2.23× | 2.89× |
| 100,000 | 0.234 | 428.03M | 0.200 | 499.46M | 0.226 | 0.97× | 1.13× |
| 1,000,000 | 2.515 | 397.63M | 2.132 | 469.02M | 2.158 | 0.86× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.111 | 1.52× |
| 1 | 5 | 0.292 | 0.467 | 1.60× |
| 1 | 10 | 0.467 | 1.077 | 2.30× |
| 10 | 1 | 0.051 | 0.096 | 1.88× |
| 10 | 5 | 0.229 | 0.468 | 2.04× |
| 10 | 10 | 0.480 | 0.995 | 2.07× |
| 100 | 1 | 0.057 | 0.108 | 1.91× |
| 100 | 5 | 0.248 | 0.534 | 2.15× |
| 100 | 10 | 0.584 | 1.047 | 1.79× |
| 1,000 | 1 | 0.056 | 0.100 | 1.78× |
| 1,000 | 5 | 0.248 | 0.545 | 2.20× |
| 1,000 | 10 | 0.564 | 1.044 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

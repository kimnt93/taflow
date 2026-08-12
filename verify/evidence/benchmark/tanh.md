# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 171.24M | 0.005 | 200.99M | 0.028 | 4.88× | 5.72× |
| 10,000 | 0.032 | 308.93M | 0.029 | 344.29M | 0.056 | 1.72× | 1.91× |
| 100,000 | 0.309 | 323.99M | 0.265 | 376.66M | 0.308 | 1.00× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.109 | 1.51× |
| 1 | 5 | 0.290 | 0.443 | 1.53× |
| 1 | 10 | 0.471 | 0.910 | 1.93× |
| 10 | 1 | 0.055 | 0.094 | 1.70× |
| 10 | 5 | 0.274 | 0.491 | 1.79× |
| 10 | 10 | 0.500 | 0.941 | 1.88× |
| 100 | 1 | 0.049 | 0.095 | 1.92× |
| 100 | 5 | 0.253 | 0.464 | 1.84× |
| 100 | 10 | 0.546 | 0.902 | 1.65× |
| 1,000 | 1 | 0.065 | 0.101 | 1.57× |
| 1,000 | 5 | 0.252 | 0.488 | 1.94× |
| 1,000 | 10 | 0.523 | 0.968 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

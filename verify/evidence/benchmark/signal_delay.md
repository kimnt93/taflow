# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.36M | 0.007 | 148.40M | 0.030 | 3.87× | 4.44× |
| 10,000 | 0.042 | 236.41M | 0.039 | 258.32M | 0.033 | 0.78× | 0.86× |
| 100,000 | 0.381 | 262.48M | 0.354 | 282.52M | 0.075 | 0.20× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.154 | 2.35× |
| 1 | 5 | 0.314 | 0.451 | 1.43× |
| 1 | 10 | 0.476 | 0.846 | 1.78× |
| 10 | 1 | 0.047 | 0.098 | 2.09× |
| 10 | 5 | 0.263 | 0.614 | 2.33× |
| 10 | 10 | 0.463 | 0.962 | 2.07× |
| 100 | 1 | 0.051 | 0.083 | 1.62× |
| 100 | 5 | 0.238 | 0.439 | 1.84× |
| 100 | 10 | 0.544 | 0.988 | 1.81× |
| 1,000 | 1 | 0.059 | 0.093 | 1.57× |
| 1,000 | 5 | 0.224 | 0.461 | 2.06× |
| 1,000 | 10 | 0.520 | 1.071 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.49M | 0.004 | 222.25M | 0.036 | 7.01× | 7.93× |
| 10,000 | 0.027 | 373.98M | 0.022 | 451.82M | 0.051 | 1.91× | 2.31× |
| 100,000 | 0.227 | 439.70M | 0.201 | 497.33M | 0.234 | 1.03× | 1.16× |
| 1,000,000 | 2.643 | 378.38M | 2.122 | 471.31M | 1.990 | 0.75× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.112 | 0.75× |
| 1 | 5 | 0.344 | 0.556 | 1.62× |
| 1 | 10 | 0.485 | 0.935 | 1.93× |
| 10 | 1 | 0.047 | 0.090 | 1.93× |
| 10 | 5 | 0.231 | 0.437 | 1.89× |
| 10 | 10 | 0.499 | 0.947 | 1.90× |
| 100 | 1 | 0.050 | 0.086 | 1.72× |
| 100 | 5 | 0.247 | 0.470 | 1.90× |
| 100 | 10 | 0.482 | 0.989 | 2.05× |
| 1,000 | 1 | 0.056 | 0.090 | 1.60× |
| 1,000 | 5 | 0.234 | 0.455 | 1.95× |
| 1,000 | 10 | 0.478 | 0.954 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

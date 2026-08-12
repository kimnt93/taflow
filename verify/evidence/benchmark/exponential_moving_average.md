# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.40M | 0.005 | 219.08M | 0.034 | 6.16× | 7.52× |
| 10,000 | 0.030 | 328.92M | 0.029 | 349.14M | 0.063 | 2.08× | 2.21× |
| 100,000 | 0.278 | 359.80M | 0.253 | 395.89M | 0.307 | 1.10× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.111 | 1.45× |
| 1 | 5 | 0.396 | 0.496 | 1.25× |
| 1 | 10 | 0.509 | 0.920 | 1.81× |
| 10 | 1 | 0.046 | 0.088 | 1.93× |
| 10 | 5 | 0.212 | 0.441 | 2.08× |
| 10 | 10 | 0.433 | 0.958 | 2.21× |
| 100 | 1 | 0.049 | 0.088 | 1.78× |
| 100 | 5 | 0.220 | 0.424 | 1.92× |
| 100 | 10 | 0.449 | 0.903 | 2.01× |
| 1,000 | 1 | 0.050 | 0.102 | 2.02× |
| 1,000 | 5 | 0.239 | 0.487 | 2.03× |
| 1,000 | 10 | 0.482 | 1.001 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

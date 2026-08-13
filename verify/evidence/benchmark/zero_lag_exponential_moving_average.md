# ZeroLagExponentialMovingAverage benchmark (`ZLEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.66M | 0.056 | 17.87M | 0.149 | 2.64× | 2.67× |
| 10,000 | 0.475 | 21.03M | 0.464 | 21.57M | 0.488 | 1.03× | 1.05× |
| 100,000 | 4.462 | 22.41M | 5.000 | 20.00M | 3.629 | 0.81× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.198 | 0.234 | 1.19× |
| 1 | 5 | 0.340 | 0.996 | 2.93× |
| 1 | 10 | 0.573 | 2.117 | 3.69× |
| 10 | 1 | 0.070 | 0.195 | 2.77× |
| 10 | 5 | 0.303 | 1.022 | 3.38× |
| 10 | 10 | 0.683 | 2.258 | 3.30× |
| 100 | 1 | 0.082 | 0.225 | 2.74× |
| 100 | 5 | 0.334 | 1.011 | 3.03× |
| 100 | 10 | 0.641 | 2.069 | 3.23× |
| 1,000 | 1 | 0.116 | 0.227 | 1.96× |
| 1,000 | 5 | 0.317 | 1.101 | 3.47× |
| 1,000 | 10 | 0.646 | 2.441 | 3.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

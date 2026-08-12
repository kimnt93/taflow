# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.98M | 0.005 | 207.19M | 0.048 | 7.17× | 9.85× |
| 10,000 | 0.033 | 303.52M | 0.026 | 377.83M | 0.133 | 4.03× | 5.01× |
| 100,000 | 1.222 | 81.83M | 0.235 | 425.64M | 1.662 | 1.36× | 7.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.157 | 1.93× |
| 1 | 5 | 0.326 | 0.514 | 1.58× |
| 1 | 10 | 0.464 | 1.011 | 2.18× |
| 10 | 1 | 0.051 | 0.095 | 1.86× |
| 10 | 5 | 0.217 | 0.505 | 2.33× |
| 10 | 10 | 0.509 | 1.012 | 1.99× |
| 100 | 1 | 0.047 | 0.100 | 2.12× |
| 100 | 5 | 0.215 | 0.485 | 2.25× |
| 100 | 10 | 0.478 | 1.047 | 2.19× |
| 1,000 | 1 | 0.055 | 0.105 | 1.92× |
| 1,000 | 5 | 0.238 | 0.558 | 2.35× |
| 1,000 | 10 | 0.492 | 1.155 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

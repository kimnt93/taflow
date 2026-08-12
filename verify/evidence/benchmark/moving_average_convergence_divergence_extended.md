# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.09M | 0.013 | 75.84M | 0.062 | 4.06× | 4.73× |
| 10,000 | 0.108 | 92.88M | 0.099 | 101.04M | 0.121 | 1.13× | 1.22× |
| 100,000 | 1.982 | 50.45M | 1.001 | 99.93M | 1.584 | 0.80× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.132 | 2.08× |
| 1 | 5 | 0.307 | 0.560 | 1.82× |
| 1 | 10 | 0.535 | 1.201 | 2.24× |
| 10 | 1 | 0.054 | 0.109 | 2.01× |
| 10 | 5 | 0.240 | 0.543 | 2.26× |
| 10 | 10 | 0.505 | 1.142 | 2.26× |
| 100 | 1 | 0.062 | 0.119 | 1.92× |
| 100 | 5 | 0.259 | 0.570 | 2.20× |
| 100 | 10 | 0.537 | 1.179 | 2.19× |
| 1,000 | 1 | 0.063 | 0.120 | 1.89× |
| 1,000 | 5 | 0.288 | 0.626 | 2.18× |
| 1,000 | 10 | 0.549 | 1.232 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

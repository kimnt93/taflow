# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.39M | 0.011 | 93.59M | 0.035 | 2.41× | 3.25× |
| 10,000 | 0.112 | 88.95M | 0.103 | 97.16M | 0.131 | 1.17× | 1.27× |
| 100,000 | 1.238 | 80.76M | 1.117 | 89.50M | 1.030 | 0.83× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.130 | 1.09× |
| 1 | 5 | 0.310 | 0.458 | 1.48× |
| 1 | 10 | 0.598 | 0.974 | 1.63× |
| 10 | 1 | 0.061 | 0.084 | 1.38× |
| 10 | 5 | 0.252 | 0.439 | 1.74× |
| 10 | 10 | 0.562 | 1.000 | 1.78× |
| 100 | 1 | 0.057 | 0.089 | 1.55× |
| 100 | 5 | 0.286 | 0.494 | 1.73× |
| 100 | 10 | 0.612 | 0.971 | 1.59× |
| 1,000 | 1 | 0.075 | 0.102 | 1.35× |
| 1,000 | 5 | 0.333 | 0.595 | 1.79× |
| 1,000 | 10 | 0.641 | 1.168 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.732 | 1.37M | 0.696 | 1.44M | 0.054 | 0.07× | 0.08× |
| 10,000 | 7.609 | 1.31M | 7.495 | 1.33M | 0.213 | 0.03× | 0.03× |
| 100,000 | 74.426 | 1.34M | 71.626 | 1.40M | 0.925 | 0.01× | 0.01× |
| 1,000,000 | 730.598 | 1.37M | 734.508 | 1.36M | 12.239 | 0.02× | 0.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.125 | 0.71× |
| 1 | 5 | 0.320 | 0.436 | 1.36× |
| 1 | 10 | 0.474 | 0.844 | 1.78× |
| 10 | 1 | 0.052 | 0.088 | 1.68× |
| 10 | 5 | 0.232 | 0.405 | 1.75× |
| 10 | 10 | 0.472 | 0.858 | 1.82× |
| 100 | 1 | 0.112 | 0.127 | 1.13× |
| 100 | 5 | 0.259 | 0.559 | 2.16× |
| 100 | 10 | 0.545 | 1.102 | 2.02× |
| 1,000 | 1 | 0.822 | 0.127 | 0.15× |
| 1,000 | 5 | 1.124 | 0.675 | 0.60× |
| 1,000 | 10 | 1.563 | 1.473 | 0.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

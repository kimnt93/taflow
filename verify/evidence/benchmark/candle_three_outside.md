# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.40M | 0.052 | 19.39M | 0.029 | 0.53× | 0.56× |
| 10,000 | 0.306 | 32.73M | 0.296 | 33.78M | 0.083 | 0.27× | 0.28× |
| 100,000 | 2.877 | 34.76M | 2.967 | 33.71M | 0.549 | 0.19× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.176 | 1.68× |
| 1 | 5 | 0.420 | 0.472 | 1.12× |
| 1 | 10 | 0.624 | 0.888 | 1.42× |
| 10 | 1 | 0.067 | 0.088 | 1.30× |
| 10 | 5 | 0.298 | 0.412 | 1.38× |
| 10 | 10 | 0.658 | 0.897 | 1.36× |
| 100 | 1 | 0.073 | 0.088 | 1.20× |
| 100 | 5 | 0.325 | 0.412 | 1.27× |
| 100 | 10 | 0.631 | 0.874 | 1.39× |
| 1,000 | 1 | 0.099 | 0.100 | 1.01× |
| 1,000 | 5 | 0.304 | 0.454 | 1.49× |
| 1,000 | 10 | 0.646 | 0.989 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

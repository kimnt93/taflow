# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 314.59M | 0.001 | 697.88M | 0.032 | 10.16× | 22.55× |
| 10,000 | 0.011 | 927.47M | 0.007 | 1.43G | 0.038 | 3.50× | 5.38× |
| 100,000 | 0.088 | 1.14G | 0.060 | 1.67G | 0.095 | 1.09× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.117 | 1.68× |
| 1 | 5 | 0.293 | 0.471 | 1.61× |
| 1 | 10 | 0.412 | 0.914 | 2.22× |
| 10 | 1 | 0.046 | 0.086 | 1.86× |
| 10 | 5 | 0.176 | 0.419 | 2.39× |
| 10 | 10 | 0.435 | 0.931 | 2.14× |
| 100 | 1 | 0.044 | 0.089 | 2.01× |
| 100 | 5 | 0.186 | 0.446 | 2.39× |
| 100 | 10 | 0.422 | 0.988 | 2.34× |
| 1,000 | 1 | 0.047 | 0.098 | 2.07× |
| 1,000 | 5 | 0.196 | 0.434 | 2.21× |
| 1,000 | 10 | 0.415 | 0.908 | 2.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

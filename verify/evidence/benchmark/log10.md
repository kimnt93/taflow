# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.96M | 0.009 | 113.29M | 0.037 | 3.64× | 4.17× |
| 10,000 | 0.088 | 113.42M | 0.083 | 120.73M | 0.111 | 1.26× | 1.34× |
| 100,000 | 0.863 | 115.86M | 0.855 | 116.96M | 0.827 | 0.96× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.125 | 2.30× |
| 1 | 5 | 0.199 | 0.431 | 2.17× |
| 1 | 10 | 0.377 | 0.955 | 2.53× |
| 10 | 1 | 0.044 | 0.087 | 1.96× |
| 10 | 5 | 0.184 | 0.438 | 2.38× |
| 10 | 10 | 0.378 | 0.939 | 2.48× |
| 100 | 1 | 0.044 | 0.098 | 2.23× |
| 100 | 5 | 0.229 | 0.485 | 2.12× |
| 100 | 10 | 0.384 | 0.870 | 2.26× |
| 1,000 | 1 | 0.061 | 0.095 | 1.55× |
| 1,000 | 5 | 0.202 | 0.473 | 2.34× |
| 1,000 | 10 | 0.472 | 1.033 | 2.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

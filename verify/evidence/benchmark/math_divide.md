# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 371.88M | 0.001 | 791.64M | 0.030 | 11.30× | 24.06× |
| 10,000 | 0.009 | 1.14G | 0.006 | 1.77G | 0.037 | 4.19× | 6.52× |
| 100,000 | 0.071 | 1.41G | 0.049 | 2.04G | 0.086 | 1.21× | 1.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.116 | 1.22× |
| 1 | 5 | 0.200 | 0.501 | 2.50× |
| 1 | 10 | 0.409 | 0.913 | 2.23× |
| 10 | 1 | 0.044 | 0.086 | 1.96× |
| 10 | 5 | 0.185 | 0.435 | 2.35× |
| 10 | 10 | 0.374 | 0.959 | 2.57× |
| 100 | 1 | 0.041 | 0.089 | 2.15× |
| 100 | 5 | 0.181 | 0.416 | 2.30× |
| 100 | 10 | 0.376 | 0.915 | 2.43× |
| 1,000 | 1 | 0.058 | 0.106 | 1.81× |
| 1,000 | 5 | 0.189 | 0.436 | 2.30× |
| 1,000 | 10 | 0.380 | 0.885 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

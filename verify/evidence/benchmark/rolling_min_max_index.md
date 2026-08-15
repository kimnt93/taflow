# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.27M | 0.009 | 115.34M | 0.041 | 4.04× | 4.74× |
| 10,000 | 0.097 | 103.10M | 0.096 | 104.00M | 0.145 | 1.50× | 1.51× |
| 100,000 | 1.485 | 67.34M | 0.972 | 102.86M | 1.233 | 0.83× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.110 | 0.69× |
| 1 | 5 | 0.305 | 0.498 | 1.63× |
| 1 | 10 | 0.414 | 0.939 | 2.27× |
| 10 | 1 | 0.043 | 0.100 | 2.34× |
| 10 | 5 | 0.178 | 0.459 | 2.58× |
| 10 | 10 | 0.382 | 0.964 | 2.52× |
| 100 | 1 | 0.043 | 0.105 | 2.41× |
| 100 | 5 | 0.184 | 0.444 | 2.42× |
| 100 | 10 | 0.386 | 0.960 | 2.49× |
| 1,000 | 1 | 0.052 | 0.106 | 2.05× |
| 1,000 | 5 | 0.196 | 0.539 | 2.75× |
| 1,000 | 10 | 0.425 | 1.086 | 2.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

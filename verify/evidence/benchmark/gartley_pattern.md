# GartleyPattern benchmark (`Gartley` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.44M | 0.012 | 81.91M | 0.231 | 10.95× | 18.90× |
| 10,000 | 0.100 | 99.90M | 0.103 | 96.69M | 1.884 | 18.82× | 18.21× |
| 100,000 | 0.964 | 103.73M | 0.928 | 107.73M | 13.005 | 13.49× | 14.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.220 | 2.59× |
| 1 | 5 | 0.358 | 0.876 | 2.45× |
| 1 | 10 | 0.541 | 1.827 | 3.38× |
| 10 | 1 | 0.072 | 0.182 | 2.52× |
| 10 | 5 | 0.266 | 1.102 | 4.15× |
| 10 | 10 | 0.543 | 1.818 | 3.35× |
| 100 | 1 | 0.067 | 0.178 | 2.68× |
| 100 | 5 | 0.257 | 1.170 | 4.55× |
| 100 | 10 | 0.587 | 1.876 | 3.19× |
| 1,000 | 1 | 0.068 | 0.306 | 4.52× |
| 1,000 | 5 | 0.261 | 1.834 | 7.02× |
| 1,000 | 10 | 0.613 | 3.136 | 5.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

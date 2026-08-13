# OrderBlock benchmark (`causal dual-scale order blocks` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.468 | 2.14M | 0.446 | 2.24M | 9.284 | 19.85× | 20.80× |
| 10,000 | 4.894 | 2.04M | 5.152 | 1.94M | 117.270 | 23.96× | 22.76× |
| 100,000 | 53.755 | 1.86M | 52.291 | 1.91M | 1236.617 | 23.00× | 23.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.189 | 0.199 | 1.05× |
| 1 | 5 | 0.593 | 0.901 | 1.52× |
| 1 | 10 | 0.760 | 1.627 | 2.14× |
| 10 | 1 | 0.090 | 0.173 | 1.92× |
| 10 | 5 | 0.375 | 0.847 | 2.26× |
| 10 | 10 | 0.813 | 1.725 | 2.12× |
| 100 | 1 | 0.124 | 0.614 | 4.96× |
| 100 | 5 | 0.372 | 3.103 | 8.34× |
| 100 | 10 | 0.793 | 6.347 | 8.00× |
| 1,000 | 1 | 0.553 | 9.519 | 17.21× |
| 1,000 | 5 | 0.927 | 89.996 | 97.06× |
| 1,000 | 10 | 2.895 | 222.667 | 76.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

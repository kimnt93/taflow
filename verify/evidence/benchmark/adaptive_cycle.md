# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.238 | 4.21M | 0.234 | 4.27M | 0.175 | 0.74× | 0.75× |
| 10,000 | 2.163 | 4.62M | 2.254 | 4.44M | 0.975 | 0.45× | 0.43× |
| 100,000 | 21.394 | 4.67M | 22.261 | 4.49M | 8.366 | 0.39× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.208 | 2.04× |
| 1 | 5 | 0.441 | 1.156 | 2.62× |
| 1 | 10 | 0.651 | 1.852 | 2.85× |
| 10 | 1 | 0.069 | 0.167 | 2.43× |
| 10 | 5 | 0.302 | 0.831 | 2.75× |
| 10 | 10 | 0.684 | 2.005 | 2.93× |
| 100 | 1 | 0.107 | 0.174 | 1.62× |
| 100 | 5 | 0.294 | 0.819 | 2.79× |
| 100 | 10 | 0.620 | 1.912 | 3.09× |
| 1,000 | 1 | 0.304 | 0.256 | 0.84× |
| 1,000 | 5 | 0.516 | 1.253 | 2.43× |
| 1,000 | 10 | 0.876 | 2.559 | 2.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

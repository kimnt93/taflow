# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.179 | 5.59M | 0.174 | 5.74M | 0.034 | 0.19× | 0.19× |
| 10,000 | 1.702 | 5.88M | 1.633 | 6.12M | 0.112 | 0.07× | 0.07× |
| 100,000 | 15.958 | 6.27M | 16.097 | 6.21M | 0.793 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.151 | 1.21× |
| 1 | 5 | 0.456 | 0.536 | 1.17× |
| 1 | 10 | 0.650 | 0.938 | 1.44× |
| 10 | 1 | 0.069 | 0.095 | 1.37× |
| 10 | 5 | 0.328 | 0.440 | 1.34× |
| 10 | 10 | 0.660 | 0.926 | 1.40× |
| 100 | 1 | 0.089 | 0.087 | 0.98× |
| 100 | 5 | 0.326 | 0.452 | 1.38× |
| 100 | 10 | 0.705 | 1.183 | 1.68× |
| 1,000 | 1 | 0.302 | 0.150 | 0.50× |
| 1,000 | 5 | 0.581 | 0.570 | 0.98× |
| 1,000 | 10 | 0.888 | 1.078 | 1.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

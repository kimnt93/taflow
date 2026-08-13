# BarsSince benchmark (`bars since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.11M | 0.028 | 36.28M | 0.114 | 3.54× | 4.13× |
| 10,000 | 0.227 | 44.09M | 0.212 | 47.15M | 1.082 | 4.77× | 5.10× |
| 100,000 | 2.219 | 45.06M | 2.088 | 47.89M | 10.255 | 4.62× | 4.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.078 | 0.63× |
| 1 | 5 | 0.358 | 0.322 | 0.90× |
| 1 | 10 | 0.535 | 0.588 | 1.10× |
| 10 | 1 | 0.059 | 0.061 | 1.04× |
| 10 | 5 | 0.268 | 0.277 | 1.03× |
| 10 | 10 | 0.557 | 0.614 | 1.10× |
| 100 | 1 | 0.066 | 0.071 | 1.08× |
| 100 | 5 | 0.280 | 0.320 | 1.14× |
| 100 | 10 | 0.597 | 0.681 | 1.14× |
| 1,000 | 1 | 0.082 | 0.167 | 2.04× |
| 1,000 | 5 | 0.281 | 0.836 | 2.97× |
| 1,000 | 10 | 0.596 | 1.757 | 2.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

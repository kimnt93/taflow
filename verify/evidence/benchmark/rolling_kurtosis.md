# RollingKurtosis benchmark (`Kurtosis` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.74M | 0.028 | 35.65M | 0.186 | 6.27× | 6.63× |
| 10,000 | 0.274 | 36.49M | 0.275 | 36.31M | 0.543 | 1.98× | 1.97× |
| 100,000 | 2.730 | 36.63M | 2.680 | 37.31M | 4.238 | 1.55× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.282 | 2.77× |
| 1 | 5 | 0.234 | 1.261 | 5.38× |
| 1 | 10 | 0.414 | 2.239 | 5.41× |
| 10 | 1 | 0.052 | 0.224 | 4.33× |
| 10 | 5 | 0.189 | 1.272 | 6.74× |
| 10 | 10 | 0.380 | 2.456 | 6.46× |
| 100 | 1 | 0.052 | 0.212 | 4.07× |
| 100 | 5 | 0.197 | 1.260 | 6.40× |
| 100 | 10 | 0.426 | 2.344 | 5.50× |
| 1,000 | 1 | 0.075 | 0.249 | 3.34× |
| 1,000 | 5 | 0.214 | 1.397 | 6.53× |
| 1,000 | 10 | 0.469 | 2.711 | 5.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

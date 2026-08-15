# VolumePriceTrend benchmark (`VolumePriceTrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 205.18M | 0.004 | 275.28M | 0.157 | 32.22× | 43.23× |
| 10,000 | 0.029 | 344.15M | 0.026 | 381.69M | 0.690 | 23.74× | 26.33× |
| 100,000 | 0.277 | 360.77M | 0.241 | 414.89M | 6.118 | 22.07× | 25.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.234 | 3.22× |
| 1 | 5 | 0.262 | 1.336 | 5.10× |
| 1 | 10 | 0.410 | 2.004 | 4.88× |
| 10 | 1 | 0.044 | 0.166 | 3.73× |
| 10 | 5 | 0.187 | 0.826 | 4.41× |
| 10 | 10 | 0.406 | 2.173 | 5.36× |
| 100 | 1 | 0.046 | 0.178 | 3.91× |
| 100 | 5 | 0.189 | 0.847 | 4.49× |
| 100 | 10 | 0.404 | 2.121 | 5.26× |
| 1,000 | 1 | 0.051 | 0.229 | 4.48× |
| 1,000 | 5 | 0.193 | 1.139 | 5.91× |
| 1,000 | 10 | 0.408 | 2.254 | 5.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.071 | 14.04M | 0.064 | 15.70M | 0.214 | 3.00× | 3.35× |
| 10,000 | 0.559 | 17.88M | 0.559 | 17.88M | 0.833 | 1.49× | 1.49× |
| 100,000 | 5.869 | 17.04M | 5.381 | 18.58M | 7.854 | 1.34× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.228 | 1.74× |
| 1 | 5 | 0.508 | 1.054 | 2.07× |
| 1 | 10 | 0.622 | 2.257 | 3.63× |
| 10 | 1 | 0.072 | 0.215 | 2.98× |
| 10 | 5 | 0.296 | 1.208 | 4.08× |
| 10 | 10 | 0.647 | 2.264 | 3.50× |
| 100 | 1 | 0.080 | 0.220 | 2.74× |
| 100 | 5 | 0.321 | 1.255 | 3.91× |
| 100 | 10 | 0.655 | 2.313 | 3.53× |
| 1,000 | 1 | 0.134 | 0.282 | 2.10× |
| 1,000 | 5 | 0.341 | 1.582 | 4.64× |
| 1,000 | 10 | 0.667 | 2.981 | 4.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

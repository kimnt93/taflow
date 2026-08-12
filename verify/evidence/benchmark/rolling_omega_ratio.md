# RollingOmegaRatio benchmark (`OmegaRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.38M | 0.031 | 32.09M | 0.227 | 6.91× | 7.30× |
| 10,000 | 0.303 | 32.98M | 0.293 | 34.14M | 0.715 | 2.36× | 2.44× |
| 100,000 | 3.075 | 32.52M | 2.899 | 34.50M | 5.711 | 1.86× | 1.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.300 | 2.94× |
| 1 | 5 | 0.231 | 1.178 | 5.11× |
| 1 | 10 | 0.505 | 2.644 | 5.24× |
| 10 | 1 | 0.057 | 0.252 | 4.43× |
| 10 | 5 | 0.244 | 1.397 | 5.71× |
| 10 | 10 | 0.524 | 2.561 | 4.89× |
| 100 | 1 | 0.055 | 0.246 | 4.43× |
| 100 | 5 | 0.242 | 1.479 | 6.11× |
| 100 | 10 | 0.535 | 2.598 | 4.85× |
| 1,000 | 1 | 0.090 | 0.357 | 3.98× |
| 1,000 | 5 | 0.275 | 1.683 | 6.11× |
| 1,000 | 10 | 0.518 | 3.214 | 6.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

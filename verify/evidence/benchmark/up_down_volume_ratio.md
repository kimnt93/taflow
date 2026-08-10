# UpDownVolumeRatio benchmark (`UpDownVolumeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.28M | 0.005 | 186.06M | 4.235 | 636.42× | 787.92× |
| 10,000 | 0.029 | 341.13M | 0.026 | 387.43M | 44.093 | 1504.13× | 1708.29× |
| 100,000 | 0.220 | 453.55M | 0.267 | 375.06M | 421.497 | 1911.70× | 1580.87× |
| 1,000,000 | 2.638 | 379.05M | 2.141 | 467.17M | 4173.499 | 1581.98× | 1949.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.290 | 1.77× |
| 1 | 5 | 0.275 | 1.214 | 4.41× |
| 1 | 10 | 0.495 | 2.232 | 4.51× |
| 10 | 1 | 0.059 | 0.353 | 6.00× |
| 10 | 5 | 0.346 | 1.674 | 4.84× |
| 10 | 10 | 0.496 | 2.564 | 5.17× |
| 100 | 1 | 0.049 | 0.630 | 12.98× |
| 100 | 5 | 0.235 | 3.116 | 13.26× |
| 100 | 10 | 0.515 | 6.668 | 12.95× |
| 1,000 | 1 | 0.059 | 4.321 | 73.72× |
| 1,000 | 5 | 0.325 | 26.480 | 81.38× |
| 1,000 | 10 | 0.642 | 52.342 | 81.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

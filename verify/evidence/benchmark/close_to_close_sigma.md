# CloseToCloseSigma benchmark (`annualized close-to-close volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.096 | 10.47M | 0.087 | 11.49M | 0.139 | 1.45× | 1.59× |
| 10,000 | 0.803 | 12.45M | 0.814 | 12.28M | 0.663 | 0.83× | 0.81× |
| 100,000 | 7.981 | 12.53M | 7.900 | 12.66M | 6.524 | 0.82× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.162 | 1.39× |
| 1 | 5 | 0.435 | 0.769 | 1.77× |
| 1 | 10 | 0.652 | 1.276 | 1.96× |
| 10 | 1 | 0.079 | 0.112 | 1.40× |
| 10 | 5 | 0.298 | 0.568 | 1.91× |
| 10 | 10 | 0.591 | 1.180 | 2.00× |
| 100 | 1 | 0.074 | 0.170 | 2.30× |
| 100 | 5 | 0.311 | 0.816 | 2.63× |
| 100 | 10 | 0.648 | 1.678 | 2.59× |
| 1,000 | 1 | 0.157 | 0.224 | 1.43× |
| 1,000 | 5 | 0.334 | 1.041 | 3.12× |
| 1,000 | 10 | 0.656 | 2.191 | 3.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

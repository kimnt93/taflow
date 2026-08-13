# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.109 | 9.18M | 0.102 | 9.79M | 0.031 | 0.28× | 0.30× |
| 10,000 | 0.919 | 10.89M | 0.924 | 10.82M | 0.081 | 0.09× | 0.09× |
| 100,000 | 10.188 | 9.82M | 9.087 | 11.01M | 0.572 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.127 | 1.23× |
| 1 | 5 | 0.404 | 0.464 | 1.15× |
| 1 | 10 | 0.678 | 0.894 | 1.32× |
| 10 | 1 | 0.066 | 0.089 | 1.34× |
| 10 | 5 | 0.314 | 0.423 | 1.35× |
| 10 | 10 | 0.625 | 0.897 | 1.43× |
| 100 | 1 | 0.076 | 0.091 | 1.21× |
| 100 | 5 | 0.315 | 0.423 | 1.34× |
| 100 | 10 | 0.673 | 0.938 | 1.39× |
| 1,000 | 1 | 0.177 | 0.102 | 0.58× |
| 1,000 | 5 | 0.325 | 0.479 | 1.47× |
| 1,000 | 10 | 0.718 | 0.983 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

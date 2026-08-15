# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 340.46M | 0.001 | 805.77M | 0.031 | 10.43× | 24.69× |
| 10,000 | 0.009 | 1.10G | 0.005 | 1.86G | 0.035 | 3.81× | 6.43× |
| 100,000 | 0.077 | 1.30G | 0.049 | 2.02G | 0.082 | 1.07× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.136 | 2.02× |
| 1 | 5 | 0.254 | 0.497 | 1.96× |
| 1 | 10 | 0.413 | 0.905 | 2.19× |
| 10 | 1 | 0.043 | 0.087 | 2.02× |
| 10 | 5 | 0.189 | 0.429 | 2.27× |
| 10 | 10 | 0.413 | 0.958 | 2.32× |
| 100 | 1 | 0.041 | 0.085 | 2.08× |
| 100 | 5 | 0.181 | 0.412 | 2.28× |
| 100 | 10 | 0.425 | 0.933 | 2.20× |
| 1,000 | 1 | 0.062 | 0.103 | 1.65× |
| 1,000 | 5 | 0.215 | 0.471 | 2.19× |
| 1,000 | 10 | 0.396 | 0.899 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

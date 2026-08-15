# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 344.87M | 0.001 | 759.68M | 0.034 | 11.63× | 25.61× |
| 10,000 | 0.009 | 1.12G | 0.005 | 1.87G | 0.037 | 4.17× | 6.94× |
| 100,000 | 0.081 | 1.24G | 0.051 | 1.98G | 0.087 | 1.08× | 1.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.122 | 1.39× |
| 1 | 5 | 0.265 | 0.467 | 1.76× |
| 1 | 10 | 0.400 | 0.907 | 2.27× |
| 10 | 1 | 0.044 | 0.085 | 1.94× |
| 10 | 5 | 0.186 | 0.411 | 2.21× |
| 10 | 10 | 0.439 | 0.932 | 2.12× |
| 100 | 1 | 0.044 | 0.086 | 1.96× |
| 100 | 5 | 0.192 | 0.436 | 2.27× |
| 100 | 10 | 0.372 | 0.936 | 2.52× |
| 1,000 | 1 | 0.055 | 0.087 | 1.56× |
| 1,000 | 5 | 0.183 | 0.439 | 2.40× |
| 1,000 | 10 | 0.395 | 0.880 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

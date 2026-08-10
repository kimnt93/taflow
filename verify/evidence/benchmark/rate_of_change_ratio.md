# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 220.72M | 0.004 | 277.81M | 0.040 | 8.75× | 11.02× |
| 10,000 | 0.034 | 291.62M | 0.018 | 568.35M | 0.039 | 1.13× | 2.20× |
| 100,000 | 0.175 | 571.49M | 0.157 | 635.35M | 0.118 | 0.67× | 0.75× |
| 1,000,000 | 1.978 | 505.53M | 1.585 | 630.87M | 1.038 | 0.52× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.119 | 1.14× |
| 1 | 5 | 0.307 | 0.448 | 1.46× |
| 1 | 10 | 0.429 | 0.907 | 2.11× |
| 10 | 1 | 0.046 | 0.091 | 1.98× |
| 10 | 5 | 0.212 | 0.440 | 2.08× |
| 10 | 10 | 0.486 | 0.892 | 1.84× |
| 100 | 1 | 0.050 | 0.087 | 1.75× |
| 100 | 5 | 0.207 | 0.419 | 2.03× |
| 100 | 10 | 0.460 | 0.908 | 1.97× |
| 1,000 | 1 | 0.050 | 0.086 | 1.73× |
| 1,000 | 5 | 0.222 | 0.436 | 1.97× |
| 1,000 | 10 | 0.490 | 0.932 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

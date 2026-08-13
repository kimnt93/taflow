# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.07M | 0.023 | 44.03M | 0.157 | 5.20× | 6.93× |
| 10,000 | 0.166 | 60.27M | 0.159 | 62.88M | 0.472 | 2.85× | 2.97× |
| 100,000 | 1.590 | 62.88M | 1.489 | 67.18M | 3.313 | 2.08× | 2.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.267 | 2.07× |
| 1 | 5 | 0.474 | 1.268 | 2.67× |
| 1 | 10 | 0.581 | 2.286 | 3.93× |
| 10 | 1 | 0.085 | 0.217 | 2.56× |
| 10 | 5 | 0.285 | 1.237 | 4.34× |
| 10 | 10 | 0.603 | 2.209 | 3.66× |
| 100 | 1 | 0.068 | 0.211 | 3.11× |
| 100 | 5 | 0.288 | 1.256 | 4.36× |
| 100 | 10 | 0.624 | 2.264 | 3.63× |
| 1,000 | 1 | 0.090 | 0.255 | 2.82× |
| 1,000 | 5 | 0.293 | 1.384 | 4.73× |
| 1,000 | 10 | 0.628 | 2.556 | 4.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

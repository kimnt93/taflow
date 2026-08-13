# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.858 | 1.17M | 0.857 | 1.17M | 0.570 | 0.66× | 0.66× |
| 10,000 | 8.461 | 1.18M | 8.503 | 1.18M | 4.021 | 0.48× | 0.47× |
| 100,000 | 86.534 | 1.16M | 88.367 | 1.13M | 43.172 | 0.50× | 0.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.321 | 2.27× |
| 1 | 5 | 0.342 | 1.359 | 3.97× |
| 1 | 10 | 0.631 | 2.581 | 4.09× |
| 10 | 1 | 0.074 | 0.243 | 3.30× |
| 10 | 5 | 0.297 | 1.394 | 4.69× |
| 10 | 10 | 0.638 | 2.670 | 4.19× |
| 100 | 1 | 0.150 | 0.287 | 1.91× |
| 100 | 5 | 0.335 | 1.585 | 4.73× |
| 100 | 10 | 0.652 | 3.009 | 4.62× |
| 1,000 | 1 | 1.026 | 7.159 | 6.97× |
| 1,000 | 5 | 1.305 | 3.817 | 2.93× |
| 1,000 | 10 | 2.119 | 7.463 | 3.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

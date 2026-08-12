# MathAsinh benchmark (`numpy.arcsinh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.87M | 0.015 | 66.78M | 0.025 | 1.58× | 1.65× |
| 10,000 | 0.131 | 76.09M | 0.131 | 76.53M | 0.143 | 1.09× | 1.10× |
| 100,000 | 1.347 | 74.24M | 1.328 | 75.31M | 2.108 | 1.57× | 1.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.067 | 0.62× |
| 1 | 5 | 0.618 | 0.384 | 0.62× |
| 1 | 10 | 0.573 | 0.759 | 1.33× |
| 10 | 1 | 0.068 | 0.082 | 1.21× |
| 10 | 5 | 0.302 | 0.405 | 1.34× |
| 10 | 10 | 0.549 | 0.626 | 1.14× |
| 100 | 1 | 0.052 | 0.060 | 1.15× |
| 100 | 5 | 0.327 | 0.420 | 1.28× |
| 100 | 10 | 0.548 | 0.688 | 1.26× |
| 1,000 | 1 | 0.064 | 0.073 | 1.14× |
| 1,000 | 5 | 0.241 | 0.316 | 1.31× |
| 1,000 | 10 | 0.495 | 0.748 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

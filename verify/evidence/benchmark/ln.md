# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.31M | 0.005 | 215.26M | 0.030 | 5.15× | 6.40× |
| 10,000 | 0.044 | 226.13M | 0.040 | 247.66M | 0.065 | 1.47× | 1.61× |
| 100,000 | 0.413 | 242.06M | 0.397 | 252.16M | 0.396 | 0.96× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.172 | 1.31× |
| 1 | 5 | 0.208 | 0.474 | 2.28× |
| 1 | 10 | 0.375 | 0.868 | 2.32× |
| 10 | 1 | 0.041 | 0.083 | 2.03× |
| 10 | 5 | 0.173 | 0.386 | 2.23× |
| 10 | 10 | 0.382 | 0.839 | 2.20× |
| 100 | 1 | 0.045 | 0.087 | 1.93× |
| 100 | 5 | 0.190 | 0.400 | 2.10× |
| 100 | 10 | 0.413 | 0.834 | 2.02× |
| 1,000 | 1 | 0.044 | 0.087 | 1.97× |
| 1,000 | 5 | 0.183 | 0.428 | 2.34× |
| 1,000 | 10 | 0.418 | 0.912 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

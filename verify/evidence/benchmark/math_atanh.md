# MathAtanh benchmark (`numpy.arctanh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.19M | 0.012 | 86.18M | 0.025 | 1.85× | 2.15× |
| 10,000 | 0.117 | 85.31M | 0.113 | 88.20M | 0.149 | 1.27× | 1.31× |
| 100,000 | 1.181 | 84.68M | 1.089 | 91.79M | 1.398 | 1.18× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.084 | 1.03× |
| 1 | 5 | 0.188 | 0.281 | 1.49× |
| 1 | 10 | 0.449 | 0.627 | 1.40× |
| 10 | 1 | 0.043 | 0.057 | 1.33× |
| 10 | 5 | 0.194 | 0.286 | 1.48× |
| 10 | 10 | 0.395 | 0.617 | 1.56× |
| 100 | 1 | 0.043 | 0.066 | 1.53× |
| 100 | 5 | 0.228 | 0.296 | 1.30× |
| 100 | 10 | 0.423 | 0.638 | 1.51× |
| 1,000 | 1 | 0.053 | 0.075 | 1.44× |
| 1,000 | 5 | 0.190 | 0.298 | 1.57× |
| 1,000 | 10 | 0.451 | 0.772 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

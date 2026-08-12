# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 307.29M | 0.002 | 424.18M | 0.014 | 4.36× | 6.02× |
| 10,000 | 0.007 | 1.36G | 0.005 | 2.07G | 0.015 | 2.08× | 3.18× |
| 100,000 | 0.057 | 1.74G | 0.032 | 3.12G | 0.041 | 0.72× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.066 | 0.76× |
| 1 | 5 | 0.297 | 0.306 | 1.03× |
| 1 | 10 | 0.477 | 0.566 | 1.19× |
| 10 | 1 | 0.044 | 0.058 | 1.33× |
| 10 | 5 | 0.206 | 0.284 | 1.38× |
| 10 | 10 | 0.475 | 0.566 | 1.19× |
| 100 | 1 | 0.045 | 0.055 | 1.21× |
| 100 | 5 | 0.225 | 0.274 | 1.22× |
| 100 | 10 | 0.476 | 0.627 | 1.32× |
| 1,000 | 1 | 0.053 | 0.059 | 1.11× |
| 1,000 | 5 | 0.221 | 0.269 | 1.22× |
| 1,000 | 10 | 0.469 | 0.602 | 1.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

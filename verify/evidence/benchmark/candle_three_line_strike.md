# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.25M | 0.004 | 247.64M | 0.033 | 4.44× | 8.19× |
| 10,000 | 0.067 | 148.38M | 0.066 | 152.01M | 0.109 | 1.62× | 1.66× |
| 100,000 | 0.752 | 132.92M | 0.691 | 144.65M | 0.768 | 1.02× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.116 | 1.00× |
| 1 | 5 | 0.286 | 0.454 | 1.59× |
| 1 | 10 | 0.406 | 0.920 | 2.26× |
| 10 | 1 | 0.039 | 0.089 | 2.27× |
| 10 | 5 | 0.174 | 0.447 | 2.56× |
| 10 | 10 | 0.407 | 0.962 | 2.37× |
| 100 | 1 | 0.051 | 0.090 | 1.78× |
| 100 | 5 | 0.209 | 0.425 | 2.04× |
| 100 | 10 | 0.400 | 0.904 | 2.26× |
| 1,000 | 1 | 0.061 | 0.095 | 1.55× |
| 1,000 | 5 | 0.233 | 0.500 | 2.14× |
| 1,000 | 10 | 0.432 | 1.011 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

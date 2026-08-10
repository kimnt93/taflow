# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 207.74M | 0.003 | 287.22M | 0.031 | 6.44× | 8.90× |
| 10,000 | 0.011 | 951.58M | 0.007 | 1.39G | 0.035 | 3.34× | 4.88× |
| 100,000 | 0.074 | 1.35G | 0.045 | 2.20G | 0.075 | 1.02× | 1.65× |
| 1,000,000 | 1.598 | 625.65M | 1.041 | 960.83M | 1.315 | 0.82× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.114 | 1.74× |
| 1 | 5 | 0.259 | 0.467 | 1.80× |
| 1 | 10 | 0.506 | 0.937 | 1.85× |
| 10 | 1 | 0.054 | 0.092 | 1.69× |
| 10 | 5 | 0.235 | 0.415 | 1.77× |
| 10 | 10 | 0.498 | 0.963 | 1.93× |
| 100 | 1 | 0.052 | 0.087 | 1.69× |
| 100 | 5 | 0.222 | 0.417 | 1.88× |
| 100 | 10 | 0.464 | 0.948 | 2.04× |
| 1,000 | 1 | 0.062 | 0.099 | 1.61× |
| 1,000 | 5 | 0.237 | 0.438 | 1.85× |
| 1,000 | 10 | 0.485 | 0.935 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

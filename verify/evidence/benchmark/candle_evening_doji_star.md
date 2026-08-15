# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.71M | 0.011 | 94.32M | 0.041 | 2.87× | 3.89× |
| 10,000 | 0.149 | 67.01M | 0.143 | 70.05M | 0.121 | 0.81× | 0.85× |
| 100,000 | 1.584 | 63.15M | 1.606 | 62.27M | 0.898 | 0.57× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.131 | 1.45× |
| 1 | 5 | 0.203 | 0.529 | 2.61× |
| 1 | 10 | 0.389 | 0.981 | 2.52× |
| 10 | 1 | 0.049 | 0.097 | 1.98× |
| 10 | 5 | 0.198 | 0.474 | 2.40× |
| 10 | 10 | 0.386 | 0.963 | 2.50× |
| 100 | 1 | 0.043 | 0.097 | 2.27× |
| 100 | 5 | 0.179 | 0.470 | 2.62× |
| 100 | 10 | 0.439 | 0.996 | 2.27× |
| 1,000 | 1 | 0.061 | 0.101 | 1.66× |
| 1,000 | 5 | 0.189 | 0.525 | 2.77× |
| 1,000 | 10 | 0.447 | 1.130 | 2.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

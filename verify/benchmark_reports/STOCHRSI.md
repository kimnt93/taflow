# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.77M | 0.025 | 40.14M | 0.060 | 2.34× | 2.42× |
| 10,000 | 0.261 | 38.29M | 0.254 | 39.44M | 0.223 | 0.85× | 0.88× |
| 100,000 | 2.652 | 37.71M | 2.521 | 39.67M | 1.695 | 0.64× | 0.67× |
| 1,000,000 | 25.573 | 39.10M | 24.833 | 40.27M | 16.191 | 0.63× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.151 | 1.63× |
| 1 | 5 | 0.288 | 0.560 | 1.94× |
| 1 | 10 | 0.509 | 1.163 | 2.29× |
| 10 | 1 | 0.049 | 0.113 | 2.29× |
| 10 | 5 | 0.234 | 0.507 | 2.16× |
| 10 | 10 | 0.517 | 1.148 | 2.22× |
| 100 | 1 | 0.060 | 0.118 | 1.96× |
| 100 | 5 | 0.243 | 0.534 | 2.20× |
| 100 | 10 | 0.549 | 1.187 | 2.16× |
| 1,000 | 1 | 0.085 | 0.132 | 1.55× |
| 1,000 | 5 | 0.289 | 0.669 | 2.32× |
| 1,000 | 10 | 0.563 | 1.301 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

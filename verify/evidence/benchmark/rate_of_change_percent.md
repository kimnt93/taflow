# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 290.25M | 0.002 | 404.35M | 0.031 | 8.99× | 12.52× |
| 10,000 | 0.019 | 513.17M | 0.016 | 615.60M | 0.041 | 2.11× | 2.53× |
| 100,000 | 0.188 | 532.49M | 0.159 | 629.96M | 0.126 | 0.67× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.148 | 1.28× |
| 1 | 5 | 0.324 | 0.490 | 1.51× |
| 1 | 10 | 0.447 | 0.968 | 2.17× |
| 10 | 1 | 0.046 | 0.089 | 1.94× |
| 10 | 5 | 0.209 | 0.428 | 2.05× |
| 10 | 10 | 0.411 | 0.981 | 2.39× |
| 100 | 1 | 0.052 | 0.106 | 2.06× |
| 100 | 5 | 0.223 | 0.510 | 2.29× |
| 100 | 10 | 0.398 | 0.934 | 2.35× |
| 1,000 | 1 | 0.053 | 0.099 | 1.87× |
| 1,000 | 5 | 0.213 | 0.519 | 2.44× |
| 1,000 | 10 | 0.400 | 0.928 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

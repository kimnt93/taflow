# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.94M | 0.007 | 151.56M | 0.052 | 6.66× | 7.95× |
| 10,000 | 0.049 | 203.35M | 0.044 | 225.74M | 0.105 | 2.14× | 2.38× |
| 100,000 | 1.860 | 53.77M | 0.784 | 127.51M | 0.898 | 0.48× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.133 | 1.31× |
| 1 | 5 | 0.298 | 0.635 | 2.13× |
| 1 | 10 | 0.576 | 1.188 | 2.06× |
| 10 | 1 | 0.058 | 0.118 | 2.01× |
| 10 | 5 | 0.231 | 0.547 | 2.36× |
| 10 | 10 | 0.542 | 1.172 | 2.16× |
| 100 | 1 | 0.051 | 0.119 | 2.34× |
| 100 | 5 | 0.255 | 0.524 | 2.05× |
| 100 | 10 | 0.515 | 1.256 | 2.44× |
| 1,000 | 1 | 0.056 | 0.114 | 2.04× |
| 1,000 | 5 | 0.263 | 0.677 | 2.58× |
| 1,000 | 10 | 0.568 | 1.293 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

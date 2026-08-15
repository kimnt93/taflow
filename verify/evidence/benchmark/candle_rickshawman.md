# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.61M | 0.004 | 279.60M | 0.039 | 5.45× | 10.91× |
| 10,000 | 0.051 | 195.01M | 0.047 | 214.11M | 0.127 | 2.47× | 2.71× |
| 100,000 | 0.675 | 148.13M | 0.666 | 150.13M | 0.941 | 1.39× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.129 | 1.68× |
| 1 | 5 | 0.259 | 0.475 | 1.83× |
| 1 | 10 | 0.404 | 0.914 | 2.26× |
| 10 | 1 | 0.056 | 0.103 | 1.85× |
| 10 | 5 | 0.223 | 0.432 | 1.94× |
| 10 | 10 | 0.457 | 0.963 | 2.11× |
| 100 | 1 | 0.044 | 0.088 | 2.03× |
| 100 | 5 | 0.201 | 0.477 | 2.38× |
| 100 | 10 | 0.417 | 0.996 | 2.39× |
| 1,000 | 1 | 0.051 | 0.098 | 1.93× |
| 1,000 | 5 | 0.209 | 0.529 | 2.53× |
| 1,000 | 10 | 0.439 | 1.064 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

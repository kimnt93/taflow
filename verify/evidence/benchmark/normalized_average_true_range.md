# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.93M | 0.006 | 156.95M | 0.041 | 5.03× | 6.38× |
| 10,000 | 0.059 | 168.82M | 0.056 | 177.29M | 0.093 | 1.58× | 1.66× |
| 100,000 | 0.567 | 176.51M | 0.532 | 188.09M | 0.623 | 1.10× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.126 | 1.44× |
| 1 | 5 | 0.328 | 0.496 | 1.51× |
| 1 | 10 | 0.376 | 0.954 | 2.54× |
| 10 | 1 | 0.044 | 0.090 | 2.04× |
| 10 | 5 | 0.184 | 0.463 | 2.52× |
| 10 | 10 | 0.402 | 0.958 | 2.38× |
| 100 | 1 | 0.041 | 0.091 | 2.21× |
| 100 | 5 | 0.180 | 0.435 | 2.42× |
| 100 | 10 | 0.382 | 1.013 | 2.65× |
| 1,000 | 1 | 0.052 | 0.099 | 1.89× |
| 1,000 | 5 | 0.200 | 0.476 | 2.38× |
| 1,000 | 10 | 0.401 | 1.023 | 2.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

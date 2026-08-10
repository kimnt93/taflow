# FibonacciExtension benchmark (`FibExtension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.79M | 0.016 | 62.37M | 0.535 | 28.22× | 33.34× |
| 10,000 | 0.172 | 58.25M | 0.162 | 61.88M | 4.552 | 26.52× | 28.17× |
| 100,000 | 1.573 | 63.57M | 1.381 | 72.44M | 48.732 | 30.98× | 35.30× |
| 1,000,000 | 17.881 | 55.93M | 16.469 | 60.72M | 525.415 | 29.38× | 31.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.216 | 2.73× |
| 1 | 5 | 0.308 | 0.863 | 2.81× |
| 1 | 10 | 0.517 | 1.856 | 3.59× |
| 10 | 1 | 0.054 | 0.167 | 3.08× |
| 10 | 5 | 0.224 | 0.846 | 3.78× |
| 10 | 10 | 0.492 | 1.878 | 3.81× |
| 100 | 1 | 0.057 | 0.246 | 4.33× |
| 100 | 5 | 0.253 | 1.067 | 4.22× |
| 100 | 10 | 0.511 | 2.341 | 4.58× |
| 1,000 | 1 | 0.077 | 0.843 | 10.98× |
| 1,000 | 5 | 0.274 | 3.546 | 12.95× |
| 1,000 | 10 | 0.527 | 7.153 | 13.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

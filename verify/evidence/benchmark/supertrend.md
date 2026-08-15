# Supertrend benchmark (`supertrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.15M | 0.018 | 56.81M | 1.666 | 86.88× | 94.65× |
| 10,000 | 0.176 | 56.85M | 0.164 | 60.90M | 2.603 | 14.80× | 15.85× |
| 100,000 | 2.029 | 49.29M | 1.634 | 61.21M | 11.585 | 5.71× | 7.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.168 | 0.271 | 1.62× |
| 1 | 5 | 0.326 | 1.133 | 3.48× |
| 1 | 10 | 0.434 | 2.466 | 5.68× |
| 10 | 1 | 0.052 | 1.762 | 34.05× |
| 10 | 5 | 0.244 | 9.370 | 38.36× |
| 10 | 10 | 0.468 | 22.344 | 47.79× |
| 100 | 1 | 0.062 | 1.992 | 31.87× |
| 100 | 5 | 0.288 | 11.800 | 41.04× |
| 100 | 10 | 0.535 | 19.861 | 37.11× |
| 1,000 | 1 | 0.079 | 2.002 | 25.41× |
| 1,000 | 5 | 0.234 | 10.056 | 42.95× |
| 1,000 | 10 | 0.469 | 19.460 | 41.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

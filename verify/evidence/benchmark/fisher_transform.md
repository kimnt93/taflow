# FisherTransform benchmark (`fisher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.70M | 0.036 | 27.54M | 1.259 | 33.62× | 34.67× |
| 10,000 | 0.427 | 23.41M | 0.407 | 24.54M | 1.768 | 4.14× | 4.34× |
| 100,000 | 3.860 | 25.90M | 3.699 | 27.03M | 6.662 | 1.73× | 1.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.163 | 0.235 | 1.44× |
| 1 | 5 | 0.275 | 1.009 | 3.67× |
| 1 | 10 | 0.501 | 1.845 | 3.69× |
| 10 | 1 | 0.070 | 1.394 | 19.87× |
| 10 | 5 | 0.241 | 6.711 | 27.89× |
| 10 | 10 | 0.505 | 13.216 | 26.16× |
| 100 | 1 | 0.062 | 1.280 | 20.70× |
| 100 | 5 | 0.248 | 6.601 | 26.62× |
| 100 | 10 | 0.528 | 13.652 | 25.88× |
| 1,000 | 1 | 0.099 | 1.354 | 13.66× |
| 1,000 | 5 | 0.248 | 7.365 | 29.70× |
| 1,000 | 10 | 0.519 | 14.503 | 27.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

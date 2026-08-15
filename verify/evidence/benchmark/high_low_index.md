# HighLowIndex benchmark (`HighLowIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.38M | 0.007 | 141.31M | 8.606 | 1061.75× | 1216.06× |
| 10,000 | 0.058 | 172.36M | 0.059 | 169.59M | 81.417 | 1403.27× | 1380.78× |
| 100,000 | 0.559 | 178.81M | 0.577 | 173.27M | 818.176 | 1462.99× | 1417.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.305 | 2.82× |
| 1 | 5 | 0.309 | 1.487 | 4.82× |
| 1 | 10 | 0.406 | 2.610 | 6.43× |
| 10 | 1 | 0.051 | 0.313 | 6.13× |
| 10 | 5 | 0.195 | 1.593 | 8.15× |
| 10 | 10 | 0.430 | 3.404 | 7.92× |
| 100 | 1 | 0.056 | 1.133 | 20.33× |
| 100 | 5 | 0.206 | 5.566 | 27.04× |
| 100 | 10 | 0.409 | 11.521 | 28.20× |
| 1,000 | 1 | 0.055 | 8.937 | 162.27× |
| 1,000 | 5 | 0.242 | 46.403 | 191.96× |
| 1,000 | 10 | 0.615 | 88.227 | 143.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

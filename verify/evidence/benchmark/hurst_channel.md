# HurstChannel benchmark (`HurstChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.30M | 0.044 | 22.70M | 0.649 | 13.81× | 14.72× |
| 10,000 | 0.401 | 24.92M | 0.388 | 25.74M | 4.617 | 11.51× | 11.89× |
| 100,000 | 3.869 | 25.85M | 3.714 | 26.93M | 48.810 | 12.62× | 13.14× |
| 1,000,000 | 39.204 | 25.51M | 37.830 | 26.43M | 532.136 | 13.57× | 14.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.309 | 4.03× |
| 1 | 5 | 0.335 | 1.432 | 4.28× |
| 1 | 10 | 0.510 | 2.941 | 5.77× |
| 10 | 1 | 0.060 | 0.271 | 4.55× |
| 10 | 5 | 0.278 | 1.579 | 5.69× |
| 10 | 10 | 0.658 | 3.108 | 4.72× |
| 100 | 1 | 0.074 | 0.305 | 4.11× |
| 100 | 5 | 0.316 | 1.835 | 5.81× |
| 100 | 10 | 0.598 | 3.255 | 5.44× |
| 1,000 | 1 | 0.097 | 0.892 | 9.19× |
| 1,000 | 5 | 0.306 | 4.066 | 13.28× |
| 1,000 | 10 | 0.563 | 7.737 | 13.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

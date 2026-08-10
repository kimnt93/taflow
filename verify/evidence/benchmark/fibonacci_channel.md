# FibonacciChannel benchmark (`FibChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.94M | 0.018 | 55.45M | 0.535 | 13.88× | 29.67× |
| 10,000 | 0.169 | 59.06M | 0.157 | 63.59M | 4.329 | 25.57× | 27.53× |
| 100,000 | 1.782 | 56.11M | 1.473 | 67.87M | 48.566 | 27.25× | 32.96× |
| 1,000,000 | 18.180 | 55.00M | 17.163 | 58.27M | 491.694 | 27.05× | 28.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.226 | 3.32× |
| 1 | 5 | 0.280 | 0.828 | 2.96× |
| 1 | 10 | 0.502 | 1.834 | 3.66× |
| 10 | 1 | 0.052 | 0.173 | 3.34× |
| 10 | 5 | 0.241 | 0.883 | 3.66× |
| 10 | 10 | 0.505 | 1.971 | 3.90× |
| 100 | 1 | 0.057 | 0.208 | 3.64× |
| 100 | 5 | 0.251 | 1.058 | 4.22× |
| 100 | 10 | 0.535 | 2.414 | 4.51× |
| 1,000 | 1 | 0.070 | 0.818 | 11.62× |
| 1,000 | 5 | 0.265 | 3.422 | 12.93× |
| 1,000 | 10 | 0.522 | 6.787 | 13.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

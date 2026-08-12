# MedianChannel benchmark (`MedianChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.162 | 6.17M | 0.167 | 5.98M | 0.959 | 5.92× | 5.74× |
| 10,000 | 1.649 | 6.06M | 1.664 | 6.01M | 7.865 | 4.77× | 4.73× |
| 100,000 | 16.614 | 6.02M | 16.176 | 6.18M | 80.483 | 4.84× | 4.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.307 | 4.67× |
| 1 | 5 | 0.404 | 1.389 | 3.44× |
| 1 | 10 | 0.467 | 2.605 | 5.58× |
| 10 | 1 | 0.053 | 0.253 | 4.80× |
| 10 | 5 | 0.242 | 1.423 | 5.88× |
| 10 | 10 | 0.494 | 2.791 | 5.65× |
| 100 | 1 | 0.070 | 0.335 | 4.81× |
| 100 | 5 | 0.240 | 1.800 | 7.51× |
| 100 | 10 | 0.505 | 3.427 | 6.78× |
| 1,000 | 1 | 0.223 | 1.242 | 5.57× |
| 1,000 | 5 | 0.354 | 5.490 | 15.50× |
| 1,000 | 10 | 0.657 | 12.193 | 18.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

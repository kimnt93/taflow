# Squeeze benchmark (`squeeze` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.182 | 5.49M | 0.173 | 5.77M | 4.263 | 23.42× | 24.61× |
| 10,000 | 1.602 | 6.24M | 1.607 | 6.22M | 6.361 | 3.97× | 3.96× |
| 100,000 | 15.974 | 6.26M | 15.804 | 6.33M | 25.613 | 1.60× | 1.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.325 | 2.59× |
| 1 | 5 | 0.592 | 1.614 | 2.72× |
| 1 | 10 | 0.759 | 3.006 | 3.96× |
| 10 | 1 | 0.082 | 0.310 | 3.78× |
| 10 | 5 | 0.357 | 1.510 | 4.23× |
| 10 | 10 | 0.724 | 3.001 | 4.14× |
| 100 | 1 | 0.101 | 4.676 | 46.31× |
| 100 | 5 | 0.421 | 23.625 | 56.12× |
| 100 | 10 | 0.822 | 47.697 | 58.06× |
| 1,000 | 1 | 0.264 | 4.786 | 18.10× |
| 1,000 | 5 | 0.532 | 24.814 | 46.65× |
| 1,000 | 10 | 0.951 | 51.776 | 54.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

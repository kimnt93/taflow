# SwingHighLow benchmark (`causal confirmed swing pivots` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.87M | 0.040 | 25.07M | 3.671 | 80.28× | 92.04× |
| 10,000 | 0.371 | 26.94M | 0.369 | 27.08M | 37.638 | 101.41× | 101.92× |
| 100,000 | 3.634 | 27.52M | 3.613 | 27.67M | 390.605 | 107.49× | 108.10× |
| 1,000,000 | 59.026 | 16.94M | 39.377 | 25.40M | 3867.310 | 65.52× | 98.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.130 | 1.03× |
| 1 | 5 | 0.264 | 0.469 | 1.77× |
| 1 | 10 | 0.471 | 0.856 | 1.82× |
| 10 | 1 | 0.052 | 0.090 | 1.72× |
| 10 | 5 | 0.232 | 0.420 | 1.81× |
| 10 | 10 | 0.499 | 0.870 | 1.74× |
| 100 | 1 | 0.058 | 0.458 | 7.94× |
| 100 | 5 | 0.240 | 2.201 | 9.16× |
| 100 | 10 | 0.508 | 4.341 | 8.55× |
| 1,000 | 1 | 0.092 | 3.857 | 42.14× |
| 1,000 | 5 | 0.311 | 20.065 | 64.48× |
| 1,000 | 10 | 0.584 | 43.673 | 74.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

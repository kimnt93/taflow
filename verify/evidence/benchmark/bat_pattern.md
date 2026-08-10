# BatPattern benchmark (`Bat` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.83M | 0.012 | 85.24M | 0.220 | 15.57× | 18.74× |
| 10,000 | 0.096 | 104.02M | 0.092 | 108.97M | 1.390 | 14.46× | 15.15× |
| 100,000 | 0.914 | 109.37M | 0.882 | 113.41M | 12.848 | 14.05× | 14.57× |
| 1,000,000 | 9.721 | 102.87M | 8.853 | 112.95M | 127.525 | 13.12× | 14.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.227 | 2.43× |
| 1 | 5 | 0.315 | 1.056 | 3.36× |
| 1 | 10 | 0.496 | 1.637 | 3.30× |
| 10 | 1 | 0.051 | 0.165 | 3.23× |
| 10 | 5 | 0.265 | 1.107 | 4.18× |
| 10 | 10 | 0.616 | 1.753 | 2.85× |
| 100 | 1 | 0.054 | 0.184 | 3.39× |
| 100 | 5 | 0.257 | 1.125 | 4.37× |
| 100 | 10 | 0.546 | 1.805 | 3.31× |
| 1,000 | 1 | 0.067 | 0.304 | 4.57× |
| 1,000 | 5 | 0.271 | 1.746 | 6.43× |
| 1,000 | 10 | 0.563 | 3.102 | 5.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# OpeningRange benchmark (`anchored opening range` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.14M | 0.007 | 146.45M | 0.528 | 51.28× | 77.30× |
| 10,000 | 0.058 | 171.16M | 0.051 | 194.41M | 4.853 | 83.07× | 94.35× |
| 100,000 | 0.539 | 185.60M | 0.483 | 206.96M | 50.799 | 94.28× | 105.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.104 | 1.43× |
| 1 | 5 | 0.299 | 0.449 | 1.50× |
| 1 | 10 | 0.384 | 0.914 | 2.38× |
| 10 | 1 | 0.043 | 0.093 | 2.14× |
| 10 | 5 | 0.208 | 0.457 | 2.20× |
| 10 | 10 | 0.379 | 0.940 | 2.48× |
| 100 | 1 | 0.042 | 0.147 | 3.51× |
| 100 | 5 | 0.192 | 0.712 | 3.72× |
| 100 | 10 | 0.411 | 1.421 | 3.46× |
| 1,000 | 1 | 0.055 | 0.608 | 11.13× |
| 1,000 | 5 | 0.223 | 3.091 | 13.84× |
| 1,000 | 10 | 0.481 | 6.697 | 13.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

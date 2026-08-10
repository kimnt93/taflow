# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.21M | 0.009 | 113.44M | 0.035 | 2.97× | 3.95× |
| 10,000 | 0.070 | 142.42M | 0.065 | 153.80M | 0.097 | 1.38× | 1.49× |
| 100,000 | 0.687 | 145.64M | 0.647 | 154.55M | 0.691 | 1.01× | 1.07× |
| 1,000,000 | 8.717 | 114.72M | 6.877 | 145.42M | 7.571 | 0.87× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.114 | 1.51× |
| 1 | 5 | 0.312 | 0.531 | 1.70× |
| 1 | 10 | 0.533 | 1.304 | 2.45× |
| 10 | 1 | 0.088 | 0.188 | 2.13× |
| 10 | 5 | 0.316 | 0.590 | 1.87× |
| 10 | 10 | 0.558 | 0.961 | 1.72× |
| 100 | 1 | 0.049 | 0.087 | 1.77× |
| 100 | 5 | 0.275 | 0.491 | 1.79× |
| 100 | 10 | 0.522 | 0.881 | 1.69× |
| 1,000 | 1 | 0.063 | 0.090 | 1.44× |
| 1,000 | 5 | 0.241 | 0.530 | 2.20× |
| 1,000 | 10 | 0.547 | 1.036 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

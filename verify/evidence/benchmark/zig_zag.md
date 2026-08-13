# ZigZag benchmark (`ZigZag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.73M | 0.042 | 23.87M | 0.533 | 10.52× | 12.73× |
| 10,000 | 0.347 | 28.78M | 0.333 | 29.99M | 3.499 | 10.07× | 10.49× |
| 100,000 | 3.736 | 26.76M | 3.345 | 29.89M | 37.692 | 10.09× | 11.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.290 | 2.68× |
| 1 | 5 | 0.397 | 1.098 | 2.76× |
| 1 | 10 | 0.621 | 2.287 | 3.68× |
| 10 | 1 | 0.078 | 0.227 | 2.92× |
| 10 | 5 | 0.304 | 1.254 | 4.12× |
| 10 | 10 | 0.595 | 2.411 | 4.05× |
| 100 | 1 | 0.085 | 0.266 | 3.14× |
| 100 | 5 | 0.329 | 1.472 | 4.48× |
| 100 | 10 | 0.668 | 2.720 | 4.07× |
| 1,000 | 1 | 0.111 | 0.738 | 6.66× |
| 1,000 | 5 | 0.315 | 3.205 | 10.19× |
| 1,000 | 10 | 0.663 | 6.449 | 9.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

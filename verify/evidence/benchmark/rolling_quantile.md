# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.19M | 0.050 | 19.83M | 0.366 | 7.75× | 7.26× |
| 10,000 | 0.495 | 20.20M | 0.471 | 21.23M | 1.890 | 3.82× | 4.01× |
| 100,000 | 5.308 | 18.84M | 4.806 | 20.81M | 18.472 | 3.48× | 3.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.289 | 4.45× |
| 1 | 5 | 0.239 | 1.427 | 5.96× |
| 1 | 10 | 0.563 | 2.723 | 4.84× |
| 10 | 1 | 0.061 | 0.243 | 3.99× |
| 10 | 5 | 0.270 | 1.570 | 5.82× |
| 10 | 10 | 0.483 | 2.805 | 5.81× |
| 100 | 1 | 0.063 | 0.286 | 4.52× |
| 100 | 5 | 0.251 | 1.519 | 6.06× |
| 100 | 10 | 0.492 | 2.846 | 5.78× |
| 1,000 | 1 | 0.103 | 0.412 | 3.99× |
| 1,000 | 5 | 0.246 | 2.333 | 9.48× |
| 1,000 | 10 | 0.536 | 4.549 | 8.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

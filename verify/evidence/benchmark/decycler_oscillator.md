# DecyclerOscillator benchmark (`DecyclerOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.57M | 0.042 | 23.62M | 0.166 | 3.59× | 3.93× |
| 10,000 | 0.351 | 28.48M | 0.372 | 26.89M | 0.475 | 1.35× | 1.28× |
| 100,000 | 3.299 | 30.31M | 3.397 | 29.44M | 3.673 | 1.11× | 1.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.165 | 0.285 | 1.73× |
| 1 | 5 | 0.436 | 1.084 | 2.48× |
| 1 | 10 | 0.602 | 2.301 | 3.82× |
| 10 | 1 | 0.071 | 0.200 | 2.82× |
| 10 | 5 | 0.288 | 1.020 | 3.54× |
| 10 | 10 | 0.603 | 2.220 | 3.68× |
| 100 | 1 | 0.067 | 0.210 | 3.12× |
| 100 | 5 | 0.287 | 1.025 | 3.57× |
| 100 | 10 | 0.623 | 2.259 | 3.63× |
| 1,000 | 1 | 0.102 | 0.238 | 2.33× |
| 1,000 | 5 | 0.285 | 1.199 | 4.20× |
| 1,000 | 10 | 0.621 | 2.660 | 4.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

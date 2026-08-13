# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.159 | 6.28M | 0.138 | 7.25M | 0.745 | 4.68× | 5.40× |
| 10,000 | 1.309 | 7.64M | 1.260 | 7.94M | 5.382 | 4.11× | 4.27× |
| 100,000 | 12.868 | 7.77M | 12.590 | 7.94M | 57.632 | 4.48× | 4.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.286 | 2.27× |
| 1 | 5 | 0.453 | 1.183 | 2.61× |
| 1 | 10 | 0.724 | 2.391 | 3.30× |
| 10 | 1 | 0.083 | 0.238 | 2.88× |
| 10 | 5 | 0.344 | 1.354 | 3.94× |
| 10 | 10 | 0.733 | 2.591 | 3.54× |
| 100 | 1 | 0.109 | 0.291 | 2.68× |
| 100 | 5 | 0.350 | 1.613 | 4.61× |
| 100 | 10 | 0.712 | 2.967 | 4.17× |
| 1,000 | 1 | 0.215 | 0.958 | 4.46× |
| 1,000 | 5 | 0.389 | 4.314 | 11.10× |
| 1,000 | 10 | 0.815 | 14.881 | 18.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

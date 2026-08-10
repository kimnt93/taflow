# TimeSegmentedVolume benchmark (`TSV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.24M | 0.009 | 115.32M | 0.248 | 23.37× | 28.59× |
| 10,000 | 0.055 | 182.76M | 0.050 | 199.71M | 0.834 | 15.23× | 16.65× |
| 100,000 | 0.510 | 196.22M | 0.461 | 216.89M | 7.747 | 15.20× | 16.80× |
| 1,000,000 | 7.445 | 134.31M | 4.953 | 201.92M | 66.749 | 8.97× | 13.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.272 | 3.82× |
| 1 | 5 | 0.322 | 1.505 | 4.67× |
| 1 | 10 | 0.554 | 3.053 | 5.51× |
| 10 | 1 | 0.063 | 0.371 | 5.92× |
| 10 | 5 | 0.309 | 1.623 | 5.25× |
| 10 | 10 | 0.605 | 2.847 | 4.71× |
| 100 | 1 | 0.066 | 0.238 | 3.62× |
| 100 | 5 | 0.255 | 1.303 | 5.11× |
| 100 | 10 | 0.516 | 2.610 | 5.06× |
| 1,000 | 1 | 0.057 | 0.283 | 4.99× |
| 1,000 | 5 | 0.256 | 1.676 | 6.54× |
| 1,000 | 10 | 0.607 | 3.153 | 5.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

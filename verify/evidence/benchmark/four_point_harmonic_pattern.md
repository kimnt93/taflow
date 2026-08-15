# FourPointHarmonicPattern benchmark (`Abcd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.77M | 0.007 | 149.23M | 0.228 | 23.17× | 33.98× |
| 10,000 | 0.093 | 107.27M | 0.085 | 117.73M | 1.479 | 15.86× | 17.41× |
| 100,000 | 0.921 | 108.59M | 0.902 | 110.91M | 13.108 | 14.23× | 14.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.213 | 2.47× |
| 1 | 5 | 0.297 | 0.840 | 2.83× |
| 1 | 10 | 0.393 | 1.735 | 4.42× |
| 10 | 1 | 0.046 | 0.165 | 3.58× |
| 10 | 5 | 0.187 | 1.066 | 5.70× |
| 10 | 10 | 0.401 | 2.068 | 5.16× |
| 100 | 1 | 0.073 | 0.263 | 3.62× |
| 100 | 5 | 0.291 | 1.821 | 6.25× |
| 100 | 10 | 0.557 | 2.756 | 4.95× |
| 1,000 | 1 | 0.083 | 0.462 | 5.58× |
| 1,000 | 5 | 0.305 | 2.327 | 7.64× |
| 1,000 | 10 | 0.565 | 3.495 | 6.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

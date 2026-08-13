# RollingGrangerCausality benchmark (`GrangerCausality` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 37.903 | 26.38K | 38.002 | 26.31K | 7.634 | 0.20× | 0.20× |
| 10,000 | 401.117 | 24.93K | 402.222 | 24.86K | 80.362 | 0.20× | 0.20× |
| 100,000 | 4056.283 | 24.65K | 4053.190 | 24.67K | 779.087 | 0.19× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.315 | 1.95× |
| 1 | 5 | 0.376 | 1.241 | 3.30× |
| 1 | 10 | 0.663 | 2.497 | 3.77× |
| 10 | 1 | 0.070 | 0.242 | 3.47× |
| 10 | 5 | 0.317 | 1.366 | 4.31× |
| 10 | 10 | 0.603 | 2.624 | 4.35× |
| 100 | 1 | 1.823 | 0.629 | 0.34× |
| 100 | 5 | 1.993 | 3.384 | 1.70× |
| 100 | 10 | 3.742 | 6.550 | 1.75× |
| 1,000 | 1 | 37.828 | 7.568 | 0.20× |
| 1,000 | 5 | 39.876 | 41.195 | 1.03× |
| 1,000 | 10 | 80.082 | 83.261 | 1.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

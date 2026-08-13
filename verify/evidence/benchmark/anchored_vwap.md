# AnchoredVolumeWeightedAveragePrice benchmark (`anchored VWAP deviation bands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.77M | 0.093 | 10.71M | 1.315 | 12.85× | 14.08× |
| 10,000 | 0.748 | 13.38M | 0.732 | 13.65M | 13.083 | 17.50× | 17.86× |
| 100,000 | 7.144 | 14.00M | 6.845 | 14.61M | 139.639 | 19.55× | 20.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.183 | 0.146 | 0.80× |
| 1 | 5 | 0.483 | 0.437 | 0.90× |
| 1 | 10 | 0.648 | 0.879 | 1.36× |
| 10 | 1 | 0.080 | 0.103 | 1.29× |
| 10 | 5 | 0.316 | 0.494 | 1.56× |
| 10 | 10 | 0.662 | 1.036 | 1.56× |
| 100 | 1 | 0.086 | 0.226 | 2.64× |
| 100 | 5 | 0.333 | 1.123 | 3.38× |
| 100 | 10 | 0.687 | 2.240 | 3.26× |
| 1,000 | 1 | 0.160 | 1.480 | 9.24× |
| 1,000 | 5 | 0.388 | 7.302 | 18.80× |
| 1,000 | 10 | 0.791 | 15.731 | 19.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

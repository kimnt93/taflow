# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.23M | 0.017 | 58.40M | 3.611 | 174.16× | 210.86× |
| 10,000 | 0.112 | 88.99M | 0.101 | 98.89M | 9.095 | 80.94× | 89.94× |
| 100,000 | 1.151 | 86.88M | 1.172 | 85.32M | 69.427 | 60.32× | 59.23× |
| 1,000,000 | 31.644 | 31.60M | 11.280 | 88.66M | 695.121 | 21.97× | 61.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 2.774 | 29.62× |
| 1 | 5 | 0.282 | 14.192 | 50.39× |
| 1 | 10 | 0.561 | 30.110 | 53.67× |
| 10 | 1 | 0.059 | 2.885 | 48.86× |
| 10 | 5 | 0.268 | 14.335 | 53.45× |
| 10 | 10 | 0.541 | 29.053 | 53.71× |
| 100 | 1 | 0.073 | 2.944 | 40.25× |
| 100 | 5 | 0.354 | 17.123 | 48.34× |
| 100 | 10 | 0.607 | 29.648 | 48.81× |
| 1,000 | 1 | 0.099 | 3.470 | 35.19× |
| 1,000 | 5 | 0.284 | 19.162 | 67.58× |
| 1,000 | 10 | 0.557 | 37.077 | 66.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

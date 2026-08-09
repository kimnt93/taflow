# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 185.11M | 0.004 | 231.00M | 0.033 | 6.13× | 7.65× |
| 10,000 | 0.025 | 407.81M | 0.021 | 477.72M | 0.042 | 1.72× | 2.01× |
| 100,000 | 0.272 | 367.79M | 0.186 | 537.76M | 0.127 | 0.47× | 0.69× |
| 1,000,000 | 2.399 | 416.84M | 1.960 | 510.19M | 1.254 | 0.52× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.103 | 0.95× |
| 1 | 5 | 0.273 | 0.545 | 2.00× |
| 1 | 10 | 0.518 | 1.008 | 1.95× |
| 10 | 1 | 0.048 | 0.089 | 1.83× |
| 10 | 5 | 0.226 | 0.457 | 2.02× |
| 10 | 10 | 0.522 | 0.997 | 1.91× |
| 100 | 1 | 0.053 | 0.114 | 2.14× |
| 100 | 5 | 0.257 | 0.479 | 1.86× |
| 100 | 10 | 0.496 | 1.011 | 2.04× |
| 1,000 | 1 | 0.052 | 0.094 | 1.83× |
| 1,000 | 5 | 0.256 | 0.470 | 1.83× |
| 1,000 | 10 | 0.526 | 0.997 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

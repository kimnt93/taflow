# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.34M | 0.012 | 85.79M | 0.039 | 3.16× | 3.33× |
| 10,000 | 0.108 | 92.47M | 0.103 | 97.00M | 0.122 | 1.12× | 1.18× |
| 100,000 | 1.018 | 98.27M | 1.024 | 97.68M | 0.939 | 0.92× | 0.92× |
| 1,000,000 | 10.813 | 92.48M | 10.162 | 98.40M | 9.633 | 0.89× | 0.95× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.038 ms**; native kernel **1.011 ms**; TA-Lib 0.943 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.303 | 0.226 | 4.43M | 931.016 | 4125.34× | 139.96× |
| 100,000 | 10 | 1.030 | 0.978 | 10.22M | 938.845 | 959.89× | 32.68× |
| 100,000 | 1,000 | 12.300 | 12.127 | 82.46M | 973.424 | 80.27× | 3.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 76.23M | 80.07M | 1.00× | 2.47M | 2.92M | 1.00× | 88.54M |
| 2 | 163.04M | 162.16M | 2.03× | 2.55M | 2.80M | 0.96× | 83.62M |
| 4 | 228.76M | 310.67M | 3.88× | 2.34M | 2.43M | 0.83× | 84.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

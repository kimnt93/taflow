# CumulativeMinimum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.00M | 0.007 | 133.65M | 0.050 | 7.02× | 6.75× |
| 10,000 | 0.045 | 219.82M | 0.042 | 237.08M | 0.091 | 2.00× | 2.15× |
| 100,000 | 0.465 | 214.99M | 0.382 | 261.56M | 0.492 | 1.06× | 1.29× |
| 1,000,000 | 4.455 | 224.47M | 4.265 | 234.44M | 4.557 | 1.02× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.269 | 2.88× |
| 1 | 5 | 0.311 | 0.677 | 2.17× |
| 1 | 10 | 0.463 | 1.250 | 2.70× |
| 10 | 1 | 0.055 | 0.223 | 4.05× |
| 10 | 5 | 0.253 | 0.624 | 2.47× |
| 10 | 10 | 0.455 | 1.216 | 2.67× |
| 100 | 1 | 0.053 | 0.166 | 3.15× |
| 100 | 5 | 0.262 | 0.590 | 2.26× |
| 100 | 10 | 0.572 | 1.327 | 2.32× |
| 1,000 | 1 | 0.056 | 0.170 | 3.02× |
| 1,000 | 5 | 0.245 | 0.576 | 2.36× |
| 1,000 | 10 | 0.551 | 1.264 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

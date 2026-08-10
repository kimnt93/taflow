# ArnaudLegouxMovingAverage benchmark (`ALMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.37M | 0.017 | 59.60M | 0.280 | 16.08× | 16.71× |
| 10,000 | 0.145 | 68.76M | 0.143 | 70.14M | 0.610 | 4.19× | 4.28× |
| 100,000 | 1.422 | 70.32M | 1.410 | 70.90M | 4.527 | 3.18× | 3.21× |
| 1,000,000 | 14.970 | 66.80M | 14.455 | 69.18M | 42.407 | 2.83× | 2.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.468 | 6.07× |
| 1 | 5 | 0.384 | 1.493 | 3.88× |
| 1 | 10 | 0.508 | 3.017 | 5.94× |
| 10 | 1 | 0.056 | 0.275 | 4.95× |
| 10 | 5 | 0.240 | 1.458 | 6.07× |
| 10 | 10 | 0.463 | 2.982 | 6.43× |
| 100 | 1 | 0.053 | 0.285 | 5.35× |
| 100 | 5 | 0.238 | 1.474 | 6.20× |
| 100 | 10 | 0.499 | 3.087 | 6.19× |
| 1,000 | 1 | 0.073 | 0.322 | 4.40× |
| 1,000 | 5 | 0.263 | 1.727 | 6.55× |
| 1,000 | 10 | 0.503 | 3.495 | 6.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

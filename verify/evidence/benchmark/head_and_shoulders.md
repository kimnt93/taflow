# HeadAndShoulders benchmark (`HeadAndShoulders` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.62M | 0.008 | 128.16M | 0.231 | 21.84× | 29.58× |
| 10,000 | 0.095 | 105.17M | 0.087 | 114.37M | 1.357 | 14.27× | 15.52× |
| 100,000 | 1.048 | 95.39M | 0.875 | 114.32M | 12.495 | 11.92× | 14.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.254 | 1.94× |
| 1 | 5 | 0.307 | 0.798 | 2.60× |
| 1 | 10 | 0.398 | 1.725 | 4.33× |
| 10 | 1 | 0.051 | 0.175 | 3.45× |
| 10 | 5 | 0.194 | 1.073 | 5.53× |
| 10 | 10 | 0.422 | 1.703 | 4.03× |
| 100 | 1 | 0.064 | 0.171 | 2.67× |
| 100 | 5 | 0.220 | 1.150 | 5.24× |
| 100 | 10 | 0.419 | 2.030 | 4.85× |
| 1,000 | 1 | 0.061 | 0.310 | 5.10× |
| 1,000 | 5 | 0.221 | 1.782 | 8.08× |
| 1,000 | 10 | 0.511 | 2.969 | 5.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

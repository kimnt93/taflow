# HeadAndShoulders benchmark (`HeadAndShoulders` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.38M | 0.012 | 84.34M | 0.222 | 15.62× | 18.71× |
| 10,000 | 0.108 | 92.33M | 0.094 | 105.95M | 1.385 | 12.79× | 14.67× |
| 100,000 | 0.978 | 102.24M | 0.957 | 104.54M | 14.014 | 14.33× | 14.65× |
| 1,000,000 | 10.053 | 99.48M | 9.750 | 102.57M | 128.265 | 12.76× | 13.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.216 | 2.36× |
| 1 | 5 | 0.392 | 1.049 | 2.68× |
| 1 | 10 | 0.503 | 1.782 | 3.54× |
| 10 | 1 | 0.087 | 0.242 | 2.79× |
| 10 | 5 | 0.267 | 1.114 | 4.18× |
| 10 | 10 | 0.540 | 1.842 | 3.41× |
| 100 | 1 | 0.073 | 0.193 | 2.64× |
| 100 | 5 | 0.264 | 1.143 | 4.33× |
| 100 | 10 | 0.541 | 1.821 | 3.36× |
| 1,000 | 1 | 0.068 | 0.306 | 4.53× |
| 1,000 | 5 | 0.261 | 1.735 | 6.64× |
| 1,000 | 10 | 0.554 | 2.996 | 5.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

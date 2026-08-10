# ThreeDrives benchmark (`ThreeDrives` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.22M | 0.011 | 87.96M | 0.217 | 15.65× | 19.06× |
| 10,000 | 0.101 | 99.36M | 0.093 | 107.76M | 1.305 | 12.96× | 14.06× |
| 100,000 | 0.920 | 108.72M | 0.887 | 112.71M | 13.246 | 14.40× | 14.93× |
| 1,000,000 | 9.839 | 101.64M | 9.360 | 106.84M | 124.833 | 12.69× | 13.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.191 | 2.04× |
| 1 | 5 | 0.270 | 1.134 | 4.21× |
| 1 | 10 | 0.600 | 1.794 | 2.99× |
| 10 | 1 | 0.054 | 0.168 | 3.09× |
| 10 | 5 | 0.282 | 1.089 | 3.86× |
| 10 | 10 | 0.541 | 1.693 | 3.13× |
| 100 | 1 | 0.059 | 0.171 | 2.90× |
| 100 | 5 | 0.258 | 1.107 | 4.29× |
| 100 | 10 | 0.524 | 1.781 | 3.40× |
| 1,000 | 1 | 0.062 | 0.297 | 4.79× |
| 1,000 | 5 | 0.265 | 1.723 | 6.51× |
| 1,000 | 10 | 0.550 | 2.986 | 5.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

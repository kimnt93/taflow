# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.57M | 0.012 | 80.19M | 0.051 | 3.55× | 4.09× |
| 10,000 | 0.072 | 138.81M | 0.070 | 143.49M | 0.135 | 1.88× | 1.94× |
| 100,000 | 0.818 | 122.28M | 0.699 | 143.05M | 0.989 | 1.21× | 1.41× |
| 1,000,000 | 8.283 | 120.72M | 8.074 | 123.86M | 9.893 | 1.19× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.160 | 2.22× |
| 1 | 5 | 0.320 | 0.617 | 1.93× |
| 1 | 10 | 0.642 | 1.132 | 1.76× |
| 10 | 1 | 0.058 | 0.114 | 1.97× |
| 10 | 5 | 0.283 | 0.488 | 1.72× |
| 10 | 10 | 0.532 | 4.540 | 8.54× |
| 100 | 1 | 1.043 | 0.416 | 0.40× |
| 100 | 5 | 0.595 | 0.699 | 1.17× |
| 100 | 10 | 0.798 | 3.777 | 4.73× |
| 1,000 | 1 | 0.093 | 0.190 | 2.05× |
| 1,000 | 5 | 0.476 | 0.612 | 1.29× |
| 1,000 | 10 | 0.667 | 1.871 | 2.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

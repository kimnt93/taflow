# AverageDailyDollarValue benchmark (`rolling average dollar volume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.05M | 0.008 | 130.84M | 0.083 | 9.35× | 10.91× |
| 10,000 | 0.051 | 195.44M | 0.049 | 204.04M | 0.289 | 5.64× | 5.89× |
| 100,000 | 0.451 | 221.82M | 0.453 | 220.85M | 2.249 | 4.99× | 4.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.113 | 1.31× |
| 1 | 5 | 0.312 | 0.545 | 1.75× |
| 1 | 10 | 0.514 | 1.111 | 2.16× |
| 10 | 1 | 0.050 | 0.107 | 2.15× |
| 10 | 5 | 0.257 | 0.561 | 2.18× |
| 10 | 10 | 0.523 | 1.165 | 2.23× |
| 100 | 1 | 0.053 | 0.147 | 2.77× |
| 100 | 5 | 0.234 | 0.694 | 2.96× |
| 100 | 10 | 0.557 | 1.460 | 2.62× |
| 1,000 | 1 | 0.059 | 0.160 | 2.70× |
| 1,000 | 5 | 0.236 | 0.820 | 3.47× |
| 1,000 | 10 | 0.513 | 1.628 | 3.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

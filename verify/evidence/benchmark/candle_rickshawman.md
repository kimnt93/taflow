# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.05M | 0.019 | 54.04M | 0.065 | 3.42× | 3.49× |
| 10,000 | 0.152 | 65.64M | 0.166 | 60.10M | 0.139 | 0.91× | 0.84× |
| 100,000 | 1.231 | 81.26M | 1.197 | 83.57M | 1.069 | 0.87× | 0.89× |
| 1,000,000 | 12.852 | 77.81M | 11.881 | 84.16M | 10.058 | 0.78× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.108 | 0.90× |
| 1 | 5 | 0.329 | 0.520 | 1.58× |
| 1 | 10 | 0.573 | 0.940 | 1.64× |
| 10 | 1 | 0.051 | 0.092 | 1.80× |
| 10 | 5 | 0.257 | 0.543 | 2.11× |
| 10 | 10 | 0.613 | 1.010 | 1.65× |
| 100 | 1 | 0.061 | 0.090 | 1.46× |
| 100 | 5 | 0.264 | 0.451 | 1.71× |
| 100 | 10 | 0.622 | 0.954 | 1.53× |
| 1,000 | 1 | 0.068 | 0.102 | 1.51× |
| 1,000 | 5 | 0.261 | 0.484 | 1.85× |
| 1,000 | 10 | 0.612 | 1.228 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

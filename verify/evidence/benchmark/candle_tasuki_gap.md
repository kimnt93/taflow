# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.75M | 0.011 | 90.34M | 0.045 | 2.98× | 4.10× |
| 10,000 | 0.146 | 68.45M | 0.153 | 65.45M | 0.185 | 1.26× | 1.21× |
| 100,000 | 1.497 | 66.82M | 1.539 | 64.96M | 1.475 | 0.99× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.172 | 2.18× |
| 1 | 5 | 0.300 | 0.501 | 1.67× |
| 1 | 10 | 0.380 | 0.935 | 2.46× |
| 10 | 1 | 0.045 | 0.094 | 2.08× |
| 10 | 5 | 0.203 | 0.464 | 2.29× |
| 10 | 10 | 0.420 | 0.960 | 2.28× |
| 100 | 1 | 0.043 | 0.096 | 2.23× |
| 100 | 5 | 0.219 | 0.459 | 2.10× |
| 100 | 10 | 0.409 | 0.963 | 2.36× |
| 1,000 | 1 | 0.060 | 0.112 | 1.86× |
| 1,000 | 5 | 0.190 | 0.514 | 2.70× |
| 1,000 | 10 | 0.418 | 1.079 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

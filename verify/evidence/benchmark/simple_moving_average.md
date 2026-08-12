# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 198.58M | 0.004 | 235.84M | 0.033 | 6.46× | 7.68× |
| 10,000 | 0.024 | 414.02M | 0.022 | 451.00M | 0.052 | 2.15× | 2.35× |
| 100,000 | 0.222 | 451.23M | 0.193 | 517.26M | 0.212 | 0.96× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.109 | 0.65× |
| 1 | 5 | 0.320 | 0.545 | 1.70× |
| 1 | 10 | 0.522 | 1.020 | 1.95× |
| 10 | 1 | 0.061 | 0.088 | 1.44× |
| 10 | 5 | 0.224 | 0.444 | 1.98× |
| 10 | 10 | 0.474 | 1.046 | 2.21× |
| 100 | 1 | 0.075 | 0.098 | 1.32× |
| 100 | 5 | 0.220 | 0.441 | 2.01× |
| 100 | 10 | 0.459 | 0.943 | 2.05× |
| 1,000 | 1 | 0.051 | 0.109 | 2.13× |
| 1,000 | 5 | 0.275 | 0.500 | 1.82× |
| 1,000 | 10 | 0.510 | 0.985 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

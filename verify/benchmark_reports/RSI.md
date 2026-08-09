# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.90M | 0.009 | 113.19M | 0.042 | 4.44× | 4.70× |
| 10,000 | 0.073 | 136.11M | 0.066 | 152.08M | 0.096 | 1.31× | 1.46× |
| 100,000 | 0.680 | 147.12M | 0.625 | 160.07M | 0.615 | 0.91× | 0.98× |
| 1,000,000 | 6.902 | 144.89M | 6.538 | 152.96M | 6.227 | 0.90× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.114 | 0.92× |
| 1 | 5 | 0.275 | 0.587 | 2.13× |
| 1 | 10 | 0.513 | 0.986 | 1.92× |
| 10 | 1 | 0.047 | 0.093 | 1.97× |
| 10 | 5 | 0.240 | 0.472 | 1.97× |
| 10 | 10 | 0.494 | 1.050 | 2.13× |
| 100 | 1 | 0.050 | 0.096 | 1.93× |
| 100 | 5 | 0.286 | 0.489 | 1.71× |
| 100 | 10 | 0.549 | 1.070 | 1.95× |
| 1,000 | 1 | 0.054 | 0.098 | 1.80× |
| 1,000 | 5 | 0.248 | 0.515 | 2.08× |
| 1,000 | 10 | 0.544 | 1.145 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

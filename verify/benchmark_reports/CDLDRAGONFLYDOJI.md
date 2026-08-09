# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.74M | 0.007 | 144.35M | 0.032 | 3.72× | 4.68× |
| 10,000 | 0.052 | 192.55M | 0.050 | 200.44M | 0.100 | 1.93× | 2.01× |
| 100,000 | 0.574 | 174.27M | 0.552 | 181.28M | 0.702 | 1.22× | 1.27× |
| 1,000,000 | 5.791 | 172.69M | 5.814 | 171.99M | 7.519 | 1.30× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.118 | 1.20× |
| 1 | 5 | 0.421 | 0.679 | 1.61× |
| 1 | 10 | 0.537 | 1.028 | 1.91× |
| 10 | 1 | 0.053 | 0.091 | 1.71× |
| 10 | 5 | 0.256 | 0.445 | 1.74× |
| 10 | 10 | 0.540 | 0.989 | 1.83× |
| 100 | 1 | 0.053 | 0.091 | 1.73× |
| 100 | 5 | 0.329 | 0.540 | 1.64× |
| 100 | 10 | 0.585 | 1.032 | 1.76× |
| 1,000 | 1 | 0.068 | 0.099 | 1.46× |
| 1,000 | 5 | 0.276 | 0.548 | 1.99× |
| 1,000 | 10 | 0.601 | 1.086 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

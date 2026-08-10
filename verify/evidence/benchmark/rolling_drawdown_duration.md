# RollingDrawdownDuration benchmark (`DrawdownDuration` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 197.07M | 0.005 | 214.39M | 0.122 | 23.98× | 26.08× |
| 10,000 | 0.025 | 398.43M | 0.023 | 441.60M | 0.428 | 17.05× | 18.90× |
| 100,000 | 0.222 | 451.32M | 0.197 | 507.11M | 3.113 | 14.05× | 15.79× |
| 1,000,000 | 2.525 | 396.03M | 1.976 | 505.95M | 32.903 | 13.03× | 16.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.212 | 1.75× |
| 1 | 5 | 0.329 | 0.804 | 2.45× |
| 1 | 10 | 0.495 | 1.714 | 3.46× |
| 10 | 1 | 0.049 | 0.154 | 3.16× |
| 10 | 5 | 0.226 | 1.131 | 5.01× |
| 10 | 10 | 0.483 | 1.645 | 3.41× |
| 100 | 1 | 0.051 | 0.164 | 3.21× |
| 100 | 5 | 0.243 | 1.119 | 4.60× |
| 100 | 10 | 0.477 | 2.109 | 4.42× |
| 1,000 | 1 | 0.060 | 0.206 | 3.45× |
| 1,000 | 5 | 0.286 | 1.324 | 4.64× |
| 1,000 | 10 | 0.492 | 2.009 | 4.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

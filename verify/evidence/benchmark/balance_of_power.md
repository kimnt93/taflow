# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.75M | 0.009 | 116.94M | 0.037 | 3.44× | 4.34× |
| 10,000 | 0.061 | 163.36M | 0.047 | 212.03M | 0.046 | 0.75× | 0.97× |
| 100,000 | 0.354 | 282.58M | 0.291 | 343.96M | 0.152 | 0.43× | 0.52× |
| 1,000,000 | 5.137 | 194.65M | 3.502 | 285.58M | 1.849 | 0.36× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.141 | 1.45× |
| 1 | 5 | 0.240 | 0.433 | 1.80× |
| 1 | 10 | 0.502 | 0.966 | 1.93× |
| 10 | 1 | 0.052 | 0.089 | 1.70× |
| 10 | 5 | 0.258 | 0.444 | 1.72× |
| 10 | 10 | 0.549 | 0.964 | 1.76× |
| 100 | 1 | 0.057 | 0.112 | 1.99× |
| 100 | 5 | 0.294 | 0.471 | 1.60× |
| 100 | 10 | 0.549 | 0.966 | 1.76× |
| 1,000 | 1 | 0.054 | 0.092 | 1.71× |
| 1,000 | 5 | 0.280 | 0.473 | 1.69× |
| 1,000 | 10 | 0.514 | 0.888 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

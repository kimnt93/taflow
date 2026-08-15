# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.41M | 0.007 | 138.11M | 0.051 | 5.63× | 7.10× |
| 10,000 | 0.065 | 153.39M | 0.059 | 169.71M | 0.092 | 1.41× | 1.56× |
| 100,000 | 1.497 | 66.80M | 0.884 | 113.10M | 0.797 | 0.53× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.146 | 1.72× |
| 1 | 5 | 0.265 | 0.585 | 2.21× |
| 1 | 10 | 0.404 | 1.205 | 2.98× |
| 10 | 1 | 0.048 | 0.109 | 2.28× |
| 10 | 5 | 0.208 | 0.610 | 2.93× |
| 10 | 10 | 0.404 | 1.094 | 2.71× |
| 100 | 1 | 0.043 | 0.107 | 2.51× |
| 100 | 5 | 0.192 | 0.540 | 2.82× |
| 100 | 10 | 0.398 | 1.151 | 2.89× |
| 1,000 | 1 | 0.052 | 0.125 | 2.42× |
| 1,000 | 5 | 0.213 | 0.572 | 2.68× |
| 1,000 | 10 | 0.444 | 1.171 | 2.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

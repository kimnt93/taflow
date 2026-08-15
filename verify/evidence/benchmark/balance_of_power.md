# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 293.04M | 0.002 | 565.75M | 0.034 | 10.07× | 19.45× |
| 10,000 | 0.011 | 918.11M | 0.007 | 1.43G | 0.041 | 3.74× | 5.83× |
| 100,000 | 0.097 | 1.03G | 0.061 | 1.63G | 0.130 | 1.35× | 2.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.141 | 0.97× |
| 1 | 5 | 0.248 | 0.478 | 1.93× |
| 1 | 10 | 0.389 | 0.910 | 2.34× |
| 10 | 1 | 0.044 | 0.087 | 2.01× |
| 10 | 5 | 0.194 | 0.462 | 2.38× |
| 10 | 10 | 0.424 | 0.910 | 2.15× |
| 100 | 1 | 0.043 | 0.083 | 1.96× |
| 100 | 5 | 0.213 | 0.432 | 2.03× |
| 100 | 10 | 0.399 | 0.926 | 2.32× |
| 1,000 | 1 | 0.045 | 0.088 | 1.95× |
| 1,000 | 5 | 0.188 | 0.458 | 2.44× |
| 1,000 | 10 | 0.385 | 0.926 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

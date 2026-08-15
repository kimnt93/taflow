# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.81M | 0.006 | 165.85M | 0.042 | 5.78× | 6.90× |
| 10,000 | 0.057 | 174.89M | 0.057 | 176.83M | 0.086 | 1.50× | 1.51× |
| 100,000 | 0.530 | 188.76M | 0.504 | 198.54M | 0.534 | 1.01× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.131 | 1.42× |
| 1 | 5 | 0.255 | 0.529 | 2.07× |
| 1 | 10 | 0.421 | 0.981 | 2.33× |
| 10 | 1 | 0.043 | 0.092 | 2.15× |
| 10 | 5 | 0.201 | 0.451 | 2.24× |
| 10 | 10 | 0.429 | 1.089 | 2.54× |
| 100 | 1 | 0.046 | 0.093 | 2.03× |
| 100 | 5 | 0.201 | 0.480 | 2.39× |
| 100 | 10 | 0.386 | 1.047 | 2.71× |
| 1,000 | 1 | 0.053 | 0.107 | 2.02× |
| 1,000 | 5 | 0.217 | 0.542 | 2.50× |
| 1,000 | 10 | 0.422 | 1.015 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

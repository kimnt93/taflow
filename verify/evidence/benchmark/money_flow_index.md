# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.79M | 0.011 | 95.03M | 0.036 | 3.02× | 3.39× |
| 10,000 | 0.062 | 162.01M | 0.059 | 169.01M | 0.106 | 1.73× | 1.80× |
| 100,000 | 0.632 | 158.27M | 0.661 | 151.26M | 0.861 | 1.36× | 1.30× |
| 1,000,000 | 7.418 | 134.81M | 6.195 | 161.41M | 8.668 | 1.17× | 1.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.179 | 0.165 | 0.92× |
| 1 | 5 | 0.432 | 0.500 | 1.16× |
| 1 | 10 | 0.498 | 0.915 | 1.84× |
| 10 | 1 | 0.049 | 0.093 | 1.88× |
| 10 | 5 | 0.228 | 0.449 | 1.97× |
| 10 | 10 | 0.496 | 0.933 | 1.88× |
| 100 | 1 | 0.050 | 0.096 | 1.92× |
| 100 | 5 | 0.243 | 0.498 | 2.05× |
| 100 | 10 | 0.531 | 1.006 | 1.89× |
| 1,000 | 1 | 0.058 | 0.100 | 1.72× |
| 1,000 | 5 | 0.256 | 0.477 | 1.86× |
| 1,000 | 10 | 0.494 | 1.007 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

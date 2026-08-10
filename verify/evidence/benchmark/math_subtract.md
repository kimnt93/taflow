# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.11M | 0.003 | 300.07M | 0.031 | 6.83× | 9.40× |
| 10,000 | 0.010 | 978.18M | 0.007 | 1.46G | 0.035 | 3.43× | 5.11× |
| 100,000 | 0.071 | 1.41G | 0.042 | 2.36G | 0.071 | 1.00× | 1.67× |
| 1,000,000 | 1.278 | 782.69M | 1.089 | 918.34M | 0.976 | 0.76× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.130 | 1.73× |
| 1 | 5 | 0.329 | 0.456 | 1.39× |
| 1 | 10 | 0.498 | 1.022 | 2.05× |
| 10 | 1 | 0.056 | 0.093 | 1.65× |
| 10 | 5 | 0.222 | 0.430 | 1.94× |
| 10 | 10 | 0.488 | 0.965 | 1.98× |
| 100 | 1 | 0.054 | 0.100 | 1.84× |
| 100 | 5 | 0.228 | 0.441 | 1.93× |
| 100 | 10 | 0.462 | 0.911 | 1.97× |
| 1,000 | 1 | 0.051 | 0.095 | 1.85× |
| 1,000 | 5 | 0.236 | 0.479 | 2.02× |
| 1,000 | 10 | 0.521 | 0.972 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

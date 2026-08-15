# ChaikinVolatility benchmark (`ChaikinVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.07M | 0.008 | 132.85M | 0.230 | 24.58× | 30.50× |
| 10,000 | 0.062 | 161.91M | 0.057 | 175.09M | 0.800 | 12.96× | 14.01× |
| 100,000 | 0.577 | 173.19M | 0.536 | 186.71M | 6.418 | 11.12× | 11.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.314 | 3.18× |
| 1 | 5 | 0.210 | 1.465 | 6.98× |
| 1 | 10 | 0.433 | 2.985 | 6.89× |
| 10 | 1 | 0.049 | 0.261 | 5.33× |
| 10 | 5 | 0.221 | 1.526 | 6.89× |
| 10 | 10 | 0.426 | 2.619 | 6.15× |
| 100 | 1 | 0.047 | 0.246 | 5.27× |
| 100 | 5 | 0.212 | 1.513 | 7.13× |
| 100 | 10 | 0.423 | 2.886 | 6.83× |
| 1,000 | 1 | 0.051 | 0.325 | 6.41× |
| 1,000 | 5 | 0.212 | 1.925 | 9.06× |
| 1,000 | 10 | 0.453 | 3.557 | 7.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

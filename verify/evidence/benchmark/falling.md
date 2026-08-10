# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.21M | 0.007 | 134.99M | 0.033 | 4.21× | 4.50× |
| 10,000 | 0.052 | 191.23M | 0.049 | 202.41M | 0.040 | 0.76× | 0.81× |
| 100,000 | 0.461 | 216.86M | 0.435 | 229.72M | 0.147 | 0.32× | 0.34× |
| 1,000,000 | 4.843 | 206.47M | 4.334 | 230.76M | 1.990 | 0.41× | 0.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.109 | 1.22× |
| 1 | 5 | 0.371 | 0.478 | 1.29× |
| 1 | 10 | 0.519 | 0.986 | 1.90× |
| 10 | 1 | 0.048 | 0.093 | 1.96× |
| 10 | 5 | 0.217 | 0.455 | 2.09× |
| 10 | 10 | 0.464 | 0.953 | 2.05× |
| 100 | 1 | 0.048 | 0.096 | 1.99× |
| 100 | 5 | 0.232 | 0.444 | 1.92× |
| 100 | 10 | 0.486 | 0.963 | 1.98× |
| 1,000 | 1 | 0.052 | 0.094 | 1.80× |
| 1,000 | 5 | 0.230 | 0.502 | 2.18× |
| 1,000 | 10 | 0.500 | 1.145 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

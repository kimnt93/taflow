# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.26M | 0.005 | 196.87M | 0.037 | 7.01× | 7.22× |
| 10,000 | 0.024 | 410.35M | 0.021 | 468.71M | 0.052 | 2.13× | 2.43× |
| 100,000 | 0.210 | 477.09M | 0.190 | 526.27M | 0.209 | 1.00× | 1.10× |
| 1,000,000 | 2.371 | 421.72M | 2.210 | 452.46M | 1.839 | 0.78× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.128 | 1.07× |
| 1 | 5 | 0.317 | 0.488 | 1.54× |
| 1 | 10 | 0.476 | 0.934 | 1.96× |
| 10 | 1 | 0.053 | 0.097 | 1.84× |
| 10 | 5 | 0.229 | 0.465 | 2.03× |
| 10 | 10 | 0.471 | 0.958 | 2.03× |
| 100 | 1 | 0.051 | 0.099 | 1.92× |
| 100 | 5 | 0.230 | 0.461 | 2.00× |
| 100 | 10 | 0.469 | 0.930 | 1.98× |
| 1,000 | 1 | 0.055 | 0.097 | 1.75× |
| 1,000 | 5 | 0.236 | 0.450 | 1.91× |
| 1,000 | 10 | 0.477 | 0.973 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

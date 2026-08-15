# RollSpread benchmark (`rolling Roll spread estimator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.77M | 0.046 | 21.78M | 0.255 | 5.30× | 5.56× |
| 10,000 | 0.446 | 22.44M | 0.435 | 22.98M | 1.301 | 2.92× | 2.99× |
| 100,000 | 4.763 | 20.99M | 4.641 | 21.55M | 13.853 | 2.91× | 2.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.147 | 1.92× |
| 1 | 5 | 0.202 | 0.537 | 2.66× |
| 1 | 10 | 0.375 | 1.351 | 3.60× |
| 10 | 1 | 0.048 | 0.121 | 2.54× |
| 10 | 5 | 0.195 | 0.552 | 2.83× |
| 10 | 10 | 0.401 | 1.119 | 2.79× |
| 100 | 1 | 0.047 | 0.242 | 5.12× |
| 100 | 5 | 0.199 | 1.247 | 6.25× |
| 100 | 10 | 0.456 | 2.598 | 5.69× |
| 1,000 | 1 | 0.102 | 0.364 | 3.58× |
| 1,000 | 5 | 0.222 | 1.488 | 6.72× |
| 1,000 | 10 | 0.417 | 3.017 | 7.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.94M | 0.038 | 26.15M | 0.021 | 0.49× | 0.56× |
| 10,000 | 0.304 | 32.90M | 0.294 | 34.00M | 0.046 | 0.15× | 0.16× |
| 100,000 | 2.865 | 34.91M | 2.935 | 34.07M | 0.284 | 0.10× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.109 | 0.87× |
| 1 | 5 | 0.366 | 0.381 | 1.04× |
| 1 | 10 | 0.599 | 0.718 | 1.20× |
| 10 | 1 | 0.063 | 0.072 | 1.13× |
| 10 | 5 | 0.285 | 0.330 | 1.16× |
| 10 | 10 | 0.594 | 0.722 | 1.22× |
| 100 | 1 | 0.064 | 0.072 | 1.12× |
| 100 | 5 | 0.283 | 0.339 | 1.20× |
| 100 | 10 | 0.623 | 0.671 | 1.08× |
| 1,000 | 1 | 0.094 | 0.078 | 0.83× |
| 1,000 | 5 | 0.288 | 0.681 | 2.37× |
| 1,000 | 10 | 0.626 | 1.130 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

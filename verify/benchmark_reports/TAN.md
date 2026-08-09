# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.51M | 0.021 | 48.00M | 0.048 | 2.15× | 2.31× |
| 10,000 | 0.221 | 45.23M | 0.218 | 45.84M | 0.243 | 1.10× | 1.11× |
| 100,000 | 2.117 | 47.23M | 2.034 | 49.16M | 1.951 | 0.92× | 0.96× |
| 1,000,000 | 20.946 | 47.74M | 20.735 | 48.23M | 19.208 | 0.92× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.123 | 1.17× |
| 1 | 5 | 0.289 | 0.460 | 1.59× |
| 1 | 10 | 0.477 | 0.885 | 1.86× |
| 10 | 1 | 0.052 | 0.088 | 1.68× |
| 10 | 5 | 0.217 | 0.401 | 1.85× |
| 10 | 10 | 0.480 | 0.896 | 1.87× |
| 100 | 1 | 0.053 | 0.091 | 1.72× |
| 100 | 5 | 0.237 | 0.419 | 1.77× |
| 100 | 10 | 0.502 | 0.897 | 1.78× |
| 1,000 | 1 | 0.072 | 0.108 | 1.49× |
| 1,000 | 5 | 0.240 | 0.521 | 2.17× |
| 1,000 | 10 | 0.506 | 1.100 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

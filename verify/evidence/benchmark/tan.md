# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.94M | 0.020 | 50.61M | 0.050 | 2.28× | 2.51× |
| 10,000 | 0.355 | 28.18M | 0.221 | 45.25M | 0.250 | 0.71× | 1.13× |
| 100,000 | 2.193 | 45.60M | 2.171 | 46.06M | 2.269 | 1.03× | 1.05× |
| 1,000,000 | 22.014 | 45.43M | 21.688 | 46.11M | 21.863 | 0.99× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.137 | 1.30× |
| 1 | 5 | 0.300 | 0.502 | 1.67× |
| 1 | 10 | 0.495 | 0.889 | 1.79× |
| 10 | 1 | 0.045 | 0.092 | 2.03× |
| 10 | 5 | 0.278 | 0.433 | 1.56× |
| 10 | 10 | 0.540 | 0.942 | 1.74× |
| 100 | 1 | 0.056 | 0.086 | 1.54× |
| 100 | 5 | 0.221 | 0.445 | 2.02× |
| 100 | 10 | 1.580 | 0.937 | 0.59× |
| 1,000 | 1 | 0.065 | 0.109 | 1.67× |
| 1,000 | 5 | 0.248 | 0.601 | 2.42× |
| 1,000 | 10 | 0.618 | 1.232 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.73M | 0.012 | 85.23M | 0.038 | 2.26× | 3.22× |
| 10,000 | 0.149 | 67.19M | 0.171 | 58.61M | 0.171 | 1.15× | 1.00× |
| 100,000 | 1.554 | 64.33M | 1.445 | 69.22M | 1.506 | 0.97× | 1.04× |
| 1,000,000 | 15.243 | 65.61M | 15.181 | 65.87M | 15.686 | 1.03× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.109 | 0.93× |
| 1 | 5 | 0.244 | 0.450 | 1.85× |
| 1 | 10 | 0.473 | 0.956 | 2.02× |
| 10 | 1 | 0.052 | 0.083 | 1.60× |
| 10 | 5 | 0.236 | 0.416 | 1.76× |
| 10 | 10 | 0.493 | 0.948 | 1.92× |
| 100 | 1 | 0.053 | 0.094 | 1.78× |
| 100 | 5 | 0.228 | 0.430 | 1.89× |
| 100 | 10 | 0.464 | 0.907 | 1.96× |
| 1,000 | 1 | 0.063 | 0.102 | 1.60× |
| 1,000 | 5 | 0.237 | 0.511 | 2.16× |
| 1,000 | 10 | 0.508 | 1.077 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 212.95M | 0.003 | 333.06M | 0.028 | 5.99× | 9.37× |
| 10,000 | 0.014 | 732.79M | 0.010 | 954.54M | 0.043 | 3.12× | 4.07× |
| 100,000 | 0.103 | 975.34M | 0.076 | 1.31G | 0.174 | 1.70× | 2.28× |
| 1,000,000 | 1.208 | 827.61M | 0.823 | 1.21G | 1.751 | 1.45× | 2.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.135 | 1.60× |
| 1 | 5 | 0.292 | 0.445 | 1.53× |
| 1 | 10 | 0.434 | 0.872 | 2.01× |
| 10 | 1 | 0.047 | 0.088 | 1.89× |
| 10 | 5 | 0.248 | 0.481 | 1.94× |
| 10 | 10 | 0.484 | 0.893 | 1.85× |
| 100 | 1 | 0.049 | 0.087 | 1.78× |
| 100 | 5 | 0.216 | 0.433 | 2.01× |
| 100 | 10 | 0.558 | 0.980 | 1.76× |
| 1,000 | 1 | 0.052 | 0.087 | 1.68× |
| 1,000 | 5 | 0.237 | 0.427 | 1.80× |
| 1,000 | 10 | 0.483 | 1.033 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

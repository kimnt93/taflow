# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.83M | 0.009 | 113.62M | 0.042 | 4.05× | 4.80× |
| 10,000 | 0.076 | 131.62M | 0.061 | 164.68M | 0.093 | 1.22× | 1.53× |
| 100,000 | 0.542 | 184.57M | 0.509 | 196.62M | 0.573 | 1.06× | 1.13× |
| 1,000,000 | 5.850 | 170.94M | 5.214 | 191.80M | 5.671 | 0.97× | 1.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.152 | 1.59× |
| 1 | 5 | 0.342 | 0.554 | 1.62× |
| 1 | 10 | 0.574 | 1.025 | 1.79× |
| 10 | 1 | 0.053 | 0.095 | 1.78× |
| 10 | 5 | 0.245 | 0.468 | 1.91× |
| 10 | 10 | 0.505 | 1.016 | 2.01× |
| 100 | 1 | 0.054 | 0.096 | 1.76× |
| 100 | 5 | 0.249 | 0.480 | 1.93× |
| 100 | 10 | 0.516 | 1.042 | 2.02× |
| 1,000 | 1 | 0.062 | 0.109 | 1.76× |
| 1,000 | 5 | 0.271 | 0.550 | 2.03× |
| 1,000 | 10 | 0.527 | 1.067 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

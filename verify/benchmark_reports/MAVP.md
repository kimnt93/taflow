# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.14M | 0.157 | 6.36M | 0.117 | 0.95× | 0.74× |
| 10,000 | 1.173 | 8.53M | 1.172 | 8.53M | 0.840 | 0.72× | 0.72× |
| 100,000 | 11.682 | 8.56M | 11.512 | 8.69M | 7.821 | 0.67× | 0.68× |
| 1,000,000 | 125.614 | 7.96M | 130.146 | 7.68M | 102.241 | 0.81× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.180 | 1.64× |
| 1 | 5 | 0.350 | 0.608 | 1.74× |
| 1 | 10 | 0.657 | 1.237 | 1.88× |
| 10 | 1 | 0.058 | 0.119 | 2.06× |
| 10 | 5 | 0.253 | 0.521 | 2.06× |
| 10 | 10 | 0.549 | 1.115 | 2.03× |
| 100 | 1 | 0.066 | 0.113 | 1.72× |
| 100 | 5 | 0.269 | 0.547 | 2.04× |
| 100 | 10 | 0.559 | 1.137 | 2.04× |
| 1,000 | 1 | 0.177 | 0.185 | 1.05× |
| 1,000 | 5 | 0.395 | 0.943 | 2.39× |
| 1,000 | 10 | 0.633 | 1.913 | 3.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

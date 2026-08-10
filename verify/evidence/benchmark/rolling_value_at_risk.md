# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.142 | 7.03M | 0.134 | 7.46M | 0.215 | 1.51× | 1.61× |
| 10,000 | 1.477 | 6.77M | 1.336 | 7.48M | 1.699 | 1.15× | 1.27× |
| 100,000 | 13.347 | 7.49M | 13.505 | 7.40M | 17.202 | 1.29× | 1.27× |
| 1,000,000 | 135.385 | 7.39M | 135.107 | 7.40M | 174.920 | 1.29× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.126 | 1.19× |
| 1 | 5 | 0.426 | 0.576 | 1.35× |
| 1 | 10 | 0.502 | 0.958 | 1.91× |
| 10 | 1 | 0.048 | 0.083 | 1.73× |
| 10 | 5 | 0.225 | 0.389 | 1.73× |
| 10 | 10 | 0.502 | 0.828 | 1.65× |
| 100 | 1 | 0.073 | 0.098 | 1.35× |
| 100 | 5 | 0.249 | 0.481 | 1.93× |
| 100 | 10 | 0.473 | 1.148 | 2.43× |
| 1,000 | 1 | 0.206 | 0.274 | 1.33× |
| 1,000 | 5 | 0.340 | 1.354 | 3.98× |
| 1,000 | 10 | 0.712 | 2.808 | 3.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

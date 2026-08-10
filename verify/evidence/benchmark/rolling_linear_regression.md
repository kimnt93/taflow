# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.69M | 0.016 | 62.52M | 0.043 | 2.35× | 2.69× |
| 10,000 | 0.132 | 75.60M | 0.130 | 76.90M | 0.153 | 1.16× | 1.18× |
| 100,000 | 1.326 | 75.42M | 1.235 | 80.94M | 1.249 | 0.94× | 1.01× |
| 1,000,000 | 13.148 | 76.06M | 12.648 | 79.07M | 13.422 | 1.02× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.110 | 0.84× |
| 1 | 5 | 0.256 | 0.481 | 1.88× |
| 1 | 10 | 0.491 | 0.954 | 1.94× |
| 10 | 1 | 0.068 | 0.093 | 1.38× |
| 10 | 5 | 0.244 | 0.514 | 2.11× |
| 10 | 10 | 0.509 | 0.960 | 1.89× |
| 100 | 1 | 0.057 | 0.092 | 1.61× |
| 100 | 5 | 0.248 | 0.473 | 1.91× |
| 100 | 10 | 0.523 | 0.981 | 1.88× |
| 1,000 | 1 | 0.070 | 0.102 | 1.46× |
| 1,000 | 5 | 0.268 | 0.690 | 2.57× |
| 1,000 | 10 | 1.260 | 1.171 | 0.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

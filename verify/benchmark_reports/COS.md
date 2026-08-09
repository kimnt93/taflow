# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.64M | 0.012 | 80.24M | 0.040 | 2.89× | 3.19× |
| 10,000 | 0.154 | 64.99M | 0.153 | 65.24M | 0.174 | 1.13× | 1.13× |
| 100,000 | 1.540 | 64.93M | 1.516 | 65.95M | 1.521 | 0.99× | 1.00× |
| 1,000,000 | 16.256 | 61.52M | 15.773 | 63.40M | 14.930 | 0.92× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.110 | 0.83× |
| 1 | 5 | 0.350 | 0.492 | 1.41× |
| 1 | 10 | 0.448 | 0.868 | 1.94× |
| 10 | 1 | 0.047 | 0.089 | 1.91× |
| 10 | 5 | 0.232 | 0.422 | 1.82× |
| 10 | 10 | 0.470 | 0.882 | 1.88× |
| 100 | 1 | 0.052 | 0.093 | 1.80× |
| 100 | 5 | 0.244 | 0.434 | 1.78× |
| 100 | 10 | 0.491 | 0.928 | 1.89× |
| 1,000 | 1 | 0.065 | 0.109 | 1.67× |
| 1,000 | 5 | 0.309 | 0.555 | 1.79× |
| 1,000 | 10 | 0.531 | 1.066 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.83M | 0.005 | 190.36M | 0.037 | 6.16× | 7.12× |
| 10,000 | 0.037 | 267.67M | 0.036 | 278.84M | 0.084 | 2.25× | 2.34× |
| 100,000 | 0.382 | 261.93M | 0.354 | 282.67M | 0.531 | 1.39× | 1.50× |
| 1,000,000 | 4.363 | 229.19M | 3.815 | 262.11M | 5.085 | 1.17× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.108 | 1.67× |
| 1 | 5 | 0.269 | 0.460 | 1.71× |
| 1 | 10 | 0.462 | 0.934 | 2.02× |
| 10 | 1 | 0.047 | 0.089 | 1.87× |
| 10 | 5 | 0.231 | 0.441 | 1.91× |
| 10 | 10 | 0.494 | 0.960 | 1.94× |
| 100 | 1 | 0.048 | 0.096 | 1.98× |
| 100 | 5 | 0.234 | 0.460 | 1.97× |
| 100 | 10 | 0.488 | 0.953 | 1.95× |
| 1,000 | 1 | 0.057 | 0.094 | 1.64× |
| 1,000 | 5 | 0.249 | 0.470 | 1.89× |
| 1,000 | 10 | 0.491 | 0.973 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

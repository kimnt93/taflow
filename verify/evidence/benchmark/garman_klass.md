# GarmanKlass benchmark (`GarmanKlassVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.01M | 0.022 | 45.80M | 0.297 | 12.46× | 13.58× |
| 10,000 | 0.179 | 55.78M | 0.166 | 60.18M | 1.512 | 8.43× | 9.10× |
| 100,000 | 1.528 | 65.47M | 1.491 | 67.07M | 13.301 | 8.71× | 8.92× |
| 1,000,000 | 16.105 | 62.09M | 15.360 | 65.10M | 136.433 | 8.47× | 8.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.285 | 2.85× |
| 1 | 5 | 0.314 | 1.382 | 4.40× |
| 1 | 10 | 0.539 | 2.770 | 5.14× |
| 10 | 1 | 0.058 | 0.240 | 4.12× |
| 10 | 5 | 0.240 | 1.450 | 6.05× |
| 10 | 10 | 0.517 | 2.620 | 5.07× |
| 100 | 1 | 0.058 | 0.280 | 4.80× |
| 100 | 5 | 0.296 | 1.729 | 5.85× |
| 100 | 10 | 0.612 | 3.156 | 5.16× |
| 1,000 | 1 | 0.077 | 0.385 | 5.03× |
| 1,000 | 5 | 0.294 | 2.294 | 7.81× |
| 1,000 | 10 | 0.594 | 4.221 | 7.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

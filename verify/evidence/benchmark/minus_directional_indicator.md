# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.37M | 0.014 | 71.45M | 0.040 | 2.71× | 2.83× |
| 10,000 | 0.118 | 85.03M | 0.095 | 105.34M | 0.099 | 0.84× | 1.04× |
| 100,000 | 0.943 | 106.04M | 0.951 | 105.13M | 0.832 | 0.88× | 0.88× |
| 1,000,000 | 10.871 | 91.99M | 9.369 | 106.73M | 6.963 | 0.64× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.128 | 1.36× |
| 1 | 5 | 0.301 | 0.470 | 1.56× |
| 1 | 10 | 0.483 | 0.978 | 2.03× |
| 10 | 1 | 0.054 | 0.091 | 1.69× |
| 10 | 5 | 0.231 | 0.452 | 1.96× |
| 10 | 10 | 0.528 | 0.986 | 1.87× |
| 100 | 1 | 0.068 | 0.098 | 1.44× |
| 100 | 5 | 0.248 | 0.483 | 1.95× |
| 100 | 10 | 0.488 | 0.960 | 1.97× |
| 1,000 | 1 | 0.062 | 0.099 | 1.59× |
| 1,000 | 5 | 0.267 | 0.541 | 2.02× |
| 1,000 | 10 | 0.513 | 1.058 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.50M | 0.007 | 144.88M | 0.041 | 4.98× | 5.94× |
| 10,000 | 0.070 | 142.14M | 0.067 | 150.08M | 0.109 | 1.55× | 1.64× |
| 100,000 | 0.671 | 148.95M | 0.663 | 150.89M | 1.072 | 1.60× | 1.62× |
| 1,000,000 | 8.218 | 121.68M | 7.212 | 138.66M | 7.594 | 0.92× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.140 | 1.61× |
| 1 | 5 | 0.364 | 0.521 | 1.43× |
| 1 | 10 | 0.514 | 1.019 | 1.98× |
| 10 | 1 | 0.050 | 0.099 | 1.96× |
| 10 | 5 | 0.229 | 0.497 | 2.17× |
| 10 | 10 | 0.532 | 0.983 | 1.85× |
| 100 | 1 | 0.050 | 0.096 | 1.93× |
| 100 | 5 | 0.231 | 0.500 | 2.17× |
| 100 | 10 | 0.488 | 0.993 | 2.03× |
| 1,000 | 1 | 0.057 | 0.102 | 1.80× |
| 1,000 | 5 | 0.230 | 0.483 | 2.10× |
| 1,000 | 10 | 0.484 | 1.050 | 2.17× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

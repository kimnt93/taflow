# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.96M | 0.008 | 118.80M | 0.034 | 3.34× | 4.10× |
| 10,000 | 0.111 | 89.69M | 0.109 | 91.40M | 0.177 | 1.59× | 1.62× |
| 100,000 | 1.175 | 85.13M | 1.147 | 87.16M | 1.495 | 1.27× | 1.30× |
| 1,000,000 | 11.595 | 86.25M | 11.606 | 86.17M | 14.870 | 1.28× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.150 | 1.15× |
| 1 | 5 | 0.354 | 0.506 | 1.43× |
| 1 | 10 | 0.498 | 0.923 | 1.85× |
| 10 | 1 | 0.061 | 0.093 | 1.52× |
| 10 | 5 | 0.330 | 0.628 | 1.90× |
| 10 | 10 | 0.670 | 0.987 | 1.47× |
| 100 | 1 | 0.054 | 0.091 | 1.69× |
| 100 | 5 | 0.268 | 0.446 | 1.67× |
| 100 | 10 | 0.511 | 0.934 | 1.83× |
| 1,000 | 1 | 0.064 | 0.108 | 1.67× |
| 1,000 | 5 | 0.263 | 0.518 | 1.97× |
| 1,000 | 10 | 0.544 | 1.077 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 220.19M | 0.004 | 256.72M | 0.030 | 6.50× | 7.58× |
| 10,000 | 0.021 | 478.62M | 0.018 | 553.94M | 0.041 | 1.97× | 2.27× |
| 100,000 | 0.188 | 530.78M | 0.153 | 651.59M | 0.124 | 0.66× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.137 | 1.38× |
| 1 | 5 | 0.308 | 0.448 | 1.45× |
| 1 | 10 | 0.490 | 0.952 | 1.94× |
| 10 | 1 | 0.051 | 0.088 | 1.71× |
| 10 | 5 | 0.237 | 0.437 | 1.84× |
| 10 | 10 | 0.454 | 0.952 | 2.10× |
| 100 | 1 | 0.054 | 0.100 | 1.85× |
| 100 | 5 | 0.220 | 0.434 | 1.97× |
| 100 | 10 | 0.471 | 0.898 | 1.91× |
| 1,000 | 1 | 0.054 | 0.090 | 1.68× |
| 1,000 | 5 | 0.241 | 0.474 | 1.96× |
| 1,000 | 10 | 0.522 | 0.986 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

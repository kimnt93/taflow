# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.54M | 0.004 | 268.54M | 0.043 | 5.50× | 11.48× |
| 10,000 | 0.082 | 121.27M | 0.079 | 126.76M | 0.125 | 1.52× | 1.59× |
| 100,000 | 0.941 | 106.28M | 0.933 | 107.22M | 0.886 | 0.94× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.142 | 1.17× |
| 1 | 5 | 0.242 | 0.478 | 1.98× |
| 1 | 10 | 0.430 | 1.000 | 2.33× |
| 10 | 1 | 0.041 | 0.096 | 2.36× |
| 10 | 5 | 0.178 | 0.461 | 2.59× |
| 10 | 10 | 0.394 | 1.082 | 2.75× |
| 100 | 1 | 0.058 | 0.107 | 1.85× |
| 100 | 5 | 0.183 | 0.480 | 2.62× |
| 100 | 10 | 0.420 | 1.000 | 2.38× |
| 1,000 | 1 | 0.064 | 0.115 | 1.80× |
| 1,000 | 5 | 0.227 | 0.539 | 2.38× |
| 1,000 | 10 | 0.452 | 1.081 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

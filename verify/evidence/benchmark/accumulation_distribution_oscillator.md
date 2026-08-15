# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.02M | 0.004 | 250.96M | 0.037 | 7.02× | 9.36× |
| 10,000 | 0.030 | 337.53M | 0.025 | 395.15M | 0.058 | 1.97× | 2.30× |
| 100,000 | 0.280 | 356.84M | 0.244 | 410.59M | 0.290 | 1.03× | 1.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.127 | 1.12× |
| 1 | 5 | 0.242 | 0.515 | 2.13× |
| 1 | 10 | 0.413 | 0.949 | 2.30× |
| 10 | 1 | 0.041 | 0.093 | 2.24× |
| 10 | 5 | 0.185 | 0.448 | 2.43× |
| 10 | 10 | 0.375 | 1.042 | 2.78× |
| 100 | 1 | 0.043 | 0.106 | 2.45× |
| 100 | 5 | 0.191 | 0.457 | 2.39× |
| 100 | 10 | 0.390 | 0.919 | 2.35× |
| 1,000 | 1 | 0.047 | 0.100 | 2.16× |
| 1,000 | 5 | 0.205 | 0.471 | 2.30× |
| 1,000 | 10 | 0.408 | 0.972 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

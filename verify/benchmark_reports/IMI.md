# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.82M | 0.016 | 61.34M | 0.097 | 5.69× | 5.93× |
| 10,000 | 0.142 | 70.28M | 0.131 | 76.55M | 0.668 | 4.69× | 5.11× |
| 100,000 | 1.332 | 75.05M | 1.261 | 79.31M | 6.240 | 4.68× | 4.95× |
| 1,000,000 | 13.869 | 72.10M | 12.908 | 77.47M | 59.530 | 4.29× | 4.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.197 | 0.231 | 1.17× |
| 1 | 5 | 0.229 | 0.479 | 2.09× |
| 1 | 10 | 0.451 | 0.956 | 2.12× |
| 10 | 1 | 0.048 | 0.097 | 2.05× |
| 10 | 5 | 0.235 | 0.507 | 2.16× |
| 10 | 10 | 0.492 | 0.993 | 2.02× |
| 100 | 1 | 0.050 | 0.099 | 1.98× |
| 100 | 5 | 0.234 | 0.481 | 2.05× |
| 100 | 10 | 0.533 | 1.030 | 1.93× |
| 1,000 | 1 | 0.059 | 0.159 | 2.67× |
| 1,000 | 5 | 0.232 | 0.752 | 3.23× |
| 1,000 | 10 | 0.512 | 1.645 | 3.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

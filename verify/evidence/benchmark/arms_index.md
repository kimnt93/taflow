# ArmsIndex benchmark (`Trin` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.96M | 0.005 | 197.29M | 9.252 | 1035.83× | 1825.31× |
| 10,000 | 0.044 | 225.35M | 0.038 | 262.15M | 91.372 | 2059.07× | 2395.33× |
| 100,000 | 0.397 | 251.78M | 0.407 | 245.54M | 884.780 | 2227.71× | 2172.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.199 | 0.408 | 2.05× |
| 1 | 5 | 0.266 | 1.375 | 5.16× |
| 1 | 10 | 0.454 | 2.824 | 6.22× |
| 10 | 1 | 0.045 | 0.306 | 6.75× |
| 10 | 5 | 0.212 | 1.598 | 7.54× |
| 10 | 10 | 0.450 | 3.780 | 8.40× |
| 100 | 1 | 0.060 | 1.142 | 19.01× |
| 100 | 5 | 0.206 | 5.830 | 28.37× |
| 100 | 10 | 0.468 | 11.689 | 24.99× |
| 1,000 | 1 | 0.054 | 8.740 | 162.62× |
| 1,000 | 5 | 0.221 | 44.818 | 202.98× |
| 1,000 | 10 | 0.531 | 89.190 | 167.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

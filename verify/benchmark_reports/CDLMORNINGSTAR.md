# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.58M | 0.008 | 127.37M | 0.037 | 3.77× | 4.73× |
| 10,000 | 0.083 | 120.91M | 0.081 | 123.98M | 0.111 | 1.35× | 1.38× |
| 100,000 | 0.889 | 112.53M | 0.851 | 117.44M | 0.821 | 0.92× | 0.96× |
| 1,000,000 | 8.946 | 111.78M | 8.950 | 111.74M | 8.117 | 0.91× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.127 | 1.21× |
| 1 | 5 | 0.315 | 0.537 | 1.70× |
| 1 | 10 | 0.522 | 1.032 | 1.98× |
| 10 | 1 | 0.054 | 0.101 | 1.87× |
| 10 | 5 | 0.269 | 0.509 | 1.90× |
| 10 | 10 | 0.547 | 1.019 | 1.86× |
| 100 | 1 | 0.057 | 0.102 | 1.77× |
| 100 | 5 | 0.260 | 0.504 | 1.94× |
| 100 | 10 | 0.553 | 1.045 | 1.89× |
| 1,000 | 1 | 0.064 | 0.111 | 1.73× |
| 1,000 | 5 | 0.270 | 0.532 | 1.97× |
| 1,000 | 10 | 0.564 | 1.121 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

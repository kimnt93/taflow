# TimeSeriesRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.79M | 0.020 | 50.76M | 0.143 | 6.98× | 7.26× |
| 10,000 | 0.174 | 57.32M | 0.177 | 56.39M | 0.764 | 4.38× | 4.31× |
| 100,000 | 1.653 | 60.50M | 1.619 | 61.75M | 6.444 | 3.90× | 3.98× |
| 1,000,000 | 16.267 | 61.48M | 16.327 | 61.25M | 64.825 | 3.99× | 3.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.130 | 1.48× |
| 1 | 5 | 0.282 | 0.558 | 1.98× |
| 1 | 10 | 0.457 | 1.082 | 2.37× |
| 10 | 1 | 0.059 | 0.111 | 1.88× |
| 10 | 5 | 0.235 | 0.522 | 2.22× |
| 10 | 10 | 0.480 | 1.069 | 2.23× |
| 100 | 1 | 0.052 | 0.158 | 3.06× |
| 100 | 5 | 0.227 | 0.770 | 3.39× |
| 100 | 10 | 0.503 | 1.587 | 3.16× |
| 1,000 | 1 | 0.070 | 0.211 | 3.01× |
| 1,000 | 5 | 0.240 | 0.959 | 3.99× |
| 1,000 | 10 | 0.519 | 2.035 | 3.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

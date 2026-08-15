# AverageDailyDollarValue benchmark (`rolling average dollar volume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.13M | 0.006 | 175.71M | 0.085 | 11.64× | 14.91× |
| 10,000 | 0.048 | 206.72M | 0.045 | 224.18M | 0.287 | 5.93× | 6.43× |
| 100,000 | 0.490 | 204.03M | 0.437 | 229.07M | 2.253 | 4.60× | 5.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.170 | 1.84× |
| 1 | 5 | 0.225 | 0.521 | 2.32× |
| 1 | 10 | 0.378 | 1.083 | 2.87× |
| 10 | 1 | 0.048 | 0.100 | 2.11× |
| 10 | 5 | 0.208 | 0.628 | 3.02× |
| 10 | 10 | 0.412 | 1.242 | 3.01× |
| 100 | 1 | 0.047 | 0.146 | 3.11× |
| 100 | 5 | 0.181 | 0.736 | 4.07× |
| 100 | 10 | 0.460 | 1.488 | 3.24× |
| 1,000 | 1 | 0.048 | 0.163 | 3.42× |
| 1,000 | 5 | 0.200 | 0.793 | 3.97× |
| 1,000 | 10 | 0.489 | 1.690 | 3.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

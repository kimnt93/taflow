# CumulativeSum benchmark (`numpy.cumsum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 232.50M | 0.003 | 292.36M | 0.017 | 3.84× | 4.83× |
| 10,000 | 0.014 | 695.95M | 0.012 | 853.14M | 0.035 | 2.45× | 3.00× |
| 100,000 | 0.125 | 802.58M | 0.107 | 935.34M | 0.219 | 1.76× | 2.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.092 | 0.76× |
| 1 | 5 | 0.262 | 0.323 | 1.23× |
| 1 | 10 | 0.454 | 0.637 | 1.40× |
| 10 | 1 | 0.059 | 0.061 | 1.03× |
| 10 | 5 | 0.219 | 0.294 | 1.34× |
| 10 | 10 | 0.505 | 0.645 | 1.28× |
| 100 | 1 | 0.061 | 0.064 | 1.06× |
| 100 | 5 | 0.219 | 0.303 | 1.39× |
| 100 | 10 | 0.488 | 0.637 | 1.31× |
| 1,000 | 1 | 0.049 | 0.074 | 1.50× |
| 1,000 | 5 | 0.274 | 0.341 | 1.24× |
| 1,000 | 10 | 0.472 | 0.688 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

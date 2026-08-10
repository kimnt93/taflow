# WedgePattern benchmark (`Wedge` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.85M | 0.011 | 94.09M | 0.233 | 13.93× | 21.90× |
| 10,000 | 0.093 | 108.08M | 0.088 | 113.61M | 1.451 | 15.68× | 16.48× |
| 100,000 | 0.877 | 114.01M | 0.845 | 118.29M | 12.925 | 14.74× | 15.29× |
| 1,000,000 | 8.956 | 111.66M | 8.443 | 118.44M | 127.641 | 14.25× | 15.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.171 | 0.215 | 1.25× |
| 1 | 5 | 0.307 | 1.059 | 3.45× |
| 1 | 10 | 0.526 | 1.690 | 3.22× |
| 10 | 1 | 0.053 | 0.170 | 3.20× |
| 10 | 5 | 0.246 | 1.082 | 4.40× |
| 10 | 10 | 0.517 | 1.674 | 3.24× |
| 100 | 1 | 0.054 | 0.183 | 3.40× |
| 100 | 5 | 0.265 | 1.143 | 4.32× |
| 100 | 10 | 0.523 | 1.862 | 3.56× |
| 1,000 | 1 | 0.066 | 0.300 | 4.52× |
| 1,000 | 5 | 0.254 | 1.746 | 6.88× |
| 1,000 | 10 | 0.540 | 2.988 | 5.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.14M | 0.009 | 108.20M | 0.202 | 19.60× | 21.83× |
| 10,000 | 0.077 | 129.09M | 0.076 | 131.26M | 0.568 | 7.33× | 7.45× |
| 100,000 | 0.796 | 125.63M | 0.732 | 136.69M | 5.239 | 6.58× | 7.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.254 | 3.74× |
| 1 | 5 | 0.311 | 1.111 | 3.57× |
| 1 | 10 | 0.498 | 2.357 | 4.73× |
| 10 | 1 | 0.060 | 0.232 | 3.87× |
| 10 | 5 | 0.220 | 1.314 | 5.99× |
| 10 | 10 | 0.549 | 2.382 | 4.34× |
| 100 | 1 | 0.066 | 0.247 | 3.76× |
| 100 | 5 | 0.251 | 8.638 | 34.47× |
| 100 | 10 | 0.652 | 2.353 | 3.61× |
| 1,000 | 1 | 0.059 | 0.254 | 4.32× |
| 1,000 | 5 | 0.253 | 1.444 | 5.71× |
| 1,000 | 10 | 0.556 | 2.700 | 4.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

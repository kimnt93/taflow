# FibonacciTimeZones benchmark (`FibTimeZones` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.92M | 0.013 | 76.06M | 0.479 | 31.06× | 36.40× |
| 10,000 | 0.162 | 61.80M | 0.151 | 66.31M | 3.667 | 22.66× | 24.31× |
| 100,000 | 1.568 | 63.79M | 1.556 | 64.28M | 41.057 | 26.19× | 26.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.212 | 3.53× |
| 1 | 5 | 0.245 | 0.824 | 3.36× |
| 1 | 10 | 0.379 | 1.936 | 5.11× |
| 10 | 1 | 0.045 | 0.178 | 3.92× |
| 10 | 5 | 0.186 | 0.839 | 4.52× |
| 10 | 10 | 0.408 | 2.013 | 4.93× |
| 100 | 1 | 0.049 | 0.210 | 4.26× |
| 100 | 5 | 0.203 | 1.012 | 4.99× |
| 100 | 10 | 0.423 | 2.453 | 5.80× |
| 1,000 | 1 | 0.066 | 0.622 | 9.43× |
| 1,000 | 5 | 0.206 | 4.255 | 20.66× |
| 1,000 | 10 | 0.539 | 8.159 | 15.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

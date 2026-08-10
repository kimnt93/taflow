# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.58M | 0.009 | 114.96M | 0.301 | 26.66× | 34.60× |
| 10,000 | 0.039 | 258.37M | 0.034 | 290.02M | 1.395 | 36.05× | 40.46× |
| 100,000 | 0.341 | 293.17M | 0.285 | 350.71M | 12.909 | 37.84× | 45.27× |
| 1,000,000 | 3.866 | 258.64M | 3.691 | 270.96M | 132.285 | 34.21× | 35.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.261 | 2.04× |
| 1 | 5 | 0.319 | 1.128 | 3.54× |
| 1 | 10 | 0.557 | 1.800 | 3.23× |
| 10 | 1 | 0.054 | 0.171 | 3.14× |
| 10 | 5 | 0.245 | 1.118 | 4.56× |
| 10 | 10 | 0.588 | 1.735 | 2.95× |
| 100 | 1 | 0.059 | 0.183 | 3.09× |
| 100 | 5 | 0.257 | 1.140 | 4.43× |
| 100 | 10 | 0.730 | 1.871 | 2.56× |
| 1,000 | 1 | 0.061 | 0.304 | 5.01× |
| 1,000 | 5 | 0.270 | 1.856 | 6.87× |
| 1,000 | 10 | 0.575 | 3.076 | 5.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

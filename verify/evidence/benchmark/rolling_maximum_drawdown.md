# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.12M | 0.044 | 22.76M | 0.227 | 5.03× | 5.18× |
| 10,000 | 0.451 | 22.16M | 0.428 | 23.38M | 1.225 | 2.71× | 2.86× |
| 100,000 | 4.477 | 22.34M | 5.288 | 18.91M | 11.395 | 2.55× | 2.15× |
| 1,000,000 | 43.179 | 23.16M | 43.400 | 23.04M | 114.886 | 2.66× | 2.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.239 | 2.25× |
| 1 | 5 | 0.266 | 0.968 | 3.64× |
| 1 | 10 | 0.488 | 2.157 | 4.42× |
| 10 | 1 | 0.051 | 0.191 | 3.71× |
| 10 | 5 | 0.223 | 0.920 | 4.13× |
| 10 | 10 | 0.463 | 2.095 | 4.52× |
| 100 | 1 | 0.057 | 0.206 | 3.60× |
| 100 | 5 | 0.238 | 0.993 | 4.17× |
| 100 | 10 | 0.500 | 2.163 | 4.32× |
| 1,000 | 1 | 0.098 | 0.314 | 3.21× |
| 1,000 | 5 | 0.230 | 1.640 | 7.15× |
| 1,000 | 10 | 0.533 | 3.410 | 6.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

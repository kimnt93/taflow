# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.50M | 0.005 | 209.57M | 0.042 | 5.58× | 8.89× |
| 10,000 | 0.084 | 119.64M | 0.072 | 139.35M | 0.156 | 1.87× | 2.18× |
| 100,000 | 0.918 | 108.94M | 0.929 | 107.64M | 1.318 | 1.44× | 1.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.163 | 2.53× |
| 1 | 5 | 0.203 | 0.463 | 2.28× |
| 1 | 10 | 0.400 | 0.889 | 2.22× |
| 10 | 1 | 0.043 | 0.088 | 2.07× |
| 10 | 5 | 0.193 | 0.439 | 2.28× |
| 10 | 10 | 0.379 | 0.881 | 2.32× |
| 100 | 1 | 0.044 | 0.093 | 2.11× |
| 100 | 5 | 0.181 | 0.432 | 2.38× |
| 100 | 10 | 0.403 | 0.893 | 2.22× |
| 1,000 | 1 | 0.050 | 0.100 | 2.01× |
| 1,000 | 5 | 0.202 | 0.500 | 2.48× |
| 1,000 | 10 | 0.411 | 1.042 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

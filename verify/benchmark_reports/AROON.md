# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.22M | 0.012 | 82.98M | 0.043 | 3.23× | 3.61× |
| 10,000 | 0.126 | 79.24M | 0.121 | 82.57M | 0.145 | 1.15× | 1.20× |
| 100,000 | 1.261 | 79.28M | 1.236 | 80.90M | 1.124 | 0.89× | 0.91× |
| 1,000,000 | 18.017 | 55.50M | 12.605 | 79.34M | 10.906 | 0.61× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.140 | 1.72× |
| 1 | 5 | 0.281 | 0.560 | 1.99× |
| 1 | 10 | 0.483 | 0.994 | 2.06× |
| 10 | 1 | 0.050 | 0.094 | 1.90× |
| 10 | 5 | 0.232 | 0.474 | 2.04× |
| 10 | 10 | 0.469 | 0.989 | 2.11× |
| 100 | 1 | 0.050 | 0.100 | 1.98× |
| 100 | 5 | 0.229 | 0.466 | 2.03× |
| 100 | 10 | 0.501 | 1.004 | 2.00× |
| 1,000 | 1 | 0.065 | 0.111 | 1.70× |
| 1,000 | 5 | 0.247 | 0.551 | 2.24× |
| 1,000 | 10 | 0.516 | 1.133 | 2.20× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.408 | 0.294 | 3.40M | 1141.977 | 3883.57× | 116.84× |
| 100,000 | 10 | 2.248 | 1.539 | 6.50M | 1136.514 | 738.40× | 22.22× |
| 100,000 | 1,000 | 81.924 | 74.493 | 13.42M | 1178.286 | 15.82× | 0.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.18M | 67.81M | 1.00× | 1.78M | 1.74M | 1.00× | 75.89M |
| 5 | 159.32M | 247.75M | 3.65× | 1.34M | 1.64M | 0.94× | 74.73M |
| 10 | 206.64M | 330.91M | 4.88× | 1.61M | 1.34M | 0.77× | 76.72M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

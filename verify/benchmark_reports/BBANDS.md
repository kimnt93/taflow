# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.11M | 0.007 | 151.75M | 0.063 | 8.08× | 9.50× |
| 10,000 | 0.053 | 189.54M | 0.046 | 216.26M | 0.111 | 2.10× | 2.39× |
| 100,000 | 0.554 | 180.47M | 0.362 | 276.07M | 0.627 | 1.13× | 1.73× |
| 1,000,000 | 17.910 | 55.84M | 8.021 | 124.67M | 8.066 | 0.45× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.146 | 2.00× |
| 1 | 5 | 0.290 | 0.703 | 2.43× |
| 1 | 10 | 0.568 | 1.452 | 2.55× |
| 10 | 1 | 0.056 | 0.114 | 2.05× |
| 10 | 5 | 0.290 | 0.677 | 2.33× |
| 10 | 10 | 0.580 | 1.446 | 2.49× |
| 100 | 1 | 0.074 | 0.189 | 2.57× |
| 100 | 5 | 0.290 | 0.612 | 2.11× |
| 100 | 10 | 0.572 | 1.428 | 2.50× |
| 1,000 | 1 | 0.056 | 0.124 | 2.22× |
| 1,000 | 5 | 0.247 | 0.583 | 2.36× |
| 1,000 | 10 | 0.563 | 1.300 | 2.31× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.410 | 0.386 | 2.59M | 574.099 | 1487.43× | 124.46× |
| 100,000 | 10 | 2.233 | 2.599 | 3.85M | 580.997 | 223.58× | 18.99× |
| 100,000 | 1,000 | 99.240 | 78.377 | 12.76M | 553.676 | 7.06× | 0.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 137.34M | 202.57M | 1.00× | 1.10M | 1.79M | 1.00× | 121.83M |
| 5 | 230.85M | 403.77M | 1.99× | 1.19M | 1.04M | 0.58× | 141.82M |
| 10 | 222.83M | 497.92M | 2.46× | 1.29M | 1.33M | 0.74× | 139.46M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

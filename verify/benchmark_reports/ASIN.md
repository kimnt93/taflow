# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.81M | 0.008 | 125.17M | 0.036 | 3.90× | 4.45× |
| 10,000 | 0.070 | 143.33M | 0.064 | 155.33M | 0.091 | 1.31× | 1.42× |
| 100,000 | 0.641 | 155.95M | 0.624 | 160.32M | 0.683 | 1.07× | 1.10× |
| 1,000,000 | 7.447 | 134.28M | 7.060 | 141.65M | 6.231 | 0.84× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.103 | 1.47× |
| 1 | 5 | 0.235 | 0.451 | 1.92× |
| 1 | 10 | 0.476 | 0.903 | 1.90× |
| 10 | 1 | 0.048 | 0.091 | 1.88× |
| 10 | 5 | 0.219 | 0.419 | 1.91× |
| 10 | 10 | 0.467 | 0.916 | 1.96× |
| 100 | 1 | 0.053 | 0.090 | 1.69× |
| 100 | 5 | 0.238 | 0.433 | 1.82× |
| 100 | 10 | 0.503 | 0.904 | 1.80× |
| 1,000 | 1 | 0.055 | 0.093 | 1.68× |
| 1,000 | 5 | 0.227 | 0.490 | 2.16× |
| 1,000 | 10 | 0.508 | 1.022 | 2.01× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.347 | 0.169 | 5.92M | 668.977 | 3962.99× | 162.34× |
| 100,000 | 10 | 1.218 | 0.601 | 16.65M | 657.536 | 1094.59× | 48.75× |
| 100,000 | 1,000 | 10.581 | 8.851 | 112.98M | 670.326 | 75.73× | 3.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 121.87M | 129.10M | 1.00× | 2.29M | 3.20M | 1.00× | 121.29M |
| 5 | 351.55M | 558.90M | 4.33× | 2.01M | 2.81M | 0.88× | 123.58M |
| 10 | 435.25M | 589.36M | 4.57× | 2.12M | 2.71M | 0.85× | 121.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

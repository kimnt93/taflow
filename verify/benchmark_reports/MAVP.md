# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.121 | 8.28M | 0.120 | 8.35M | 0.111 | 0.92× | 0.93× |
| 10,000 | 1.144 | 8.74M | 1.140 | 8.77M | 0.764 | 0.67× | 0.67× |
| 100,000 | 11.130 | 8.99M | 11.050 | 9.05M | 7.615 | 0.68× | 0.69× |
| 1,000,000 | 113.004 | 8.85M | 113.962 | 8.77M | 89.274 | 0.79× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.139 | 1.42× |
| 1 | 5 | 0.383 | 0.574 | 1.50× |
| 1 | 10 | 0.537 | 1.055 | 1.96× |
| 10 | 1 | 0.057 | 0.109 | 1.92× |
| 10 | 5 | 0.243 | 0.490 | 2.02× |
| 10 | 10 | 0.515 | 1.041 | 2.02× |
| 100 | 1 | 0.063 | 0.111 | 1.76× |
| 100 | 5 | 0.250 | 0.522 | 2.09× |
| 100 | 10 | 0.552 | 1.075 | 1.95× |
| 1,000 | 1 | 0.172 | 0.183 | 1.07× |
| 1,000 | 5 | 0.353 | 0.878 | 2.48× |
| 1,000 | 10 | 0.636 | 1.795 | 2.82× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.435 | 0.345 | 2.90M | 7462.596 | 21621.66× | 113.06× |
| 100,000 | 10 | 3.003 | 2.063 | 4.85M | 7256.678 | 3516.82× | 18.79× |
| 100,000 | 1,000 | 116.896 | 112.953 | 8.85M | 7292.382 | 64.56× | 1.01× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.32M | 8.80M | 1.00× | 1.70M | 2.37M | 1.00× | 12.56M |
| 5 | 37.41M | 39.18M | 4.45× | 1.76M | 1.91M | 0.81× | 12.18M |
| 10 | 39.99M | 40.79M | 4.63× | 1.69M | 1.86M | 0.78× | 12.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

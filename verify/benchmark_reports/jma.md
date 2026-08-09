# JurikMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.103 | 9.70M | 0.103 | 9.75M | 23.421 | 227.29× | 228.41× |
| 10,000 | 0.938 | 10.67M | 0.936 | 10.69M | 241.079 | 257.13× | 257.67× |
| 100,000 | 10.540 | 9.49M | 9.706 | 10.30M | 2144.780 | 203.50× | 220.98× |
| 1,000,000 | 97.457 | 10.26M | 92.370 | 10.83M | 25052.299 | 257.06× | 271.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.135 | 1.79× |
| 1 | 5 | 0.261 | 0.502 | 1.93× |
| 1 | 10 | 0.554 | 0.952 | 1.72× |
| 10 | 1 | 0.056 | 0.427 | 7.66× |
| 10 | 5 | 0.255 | 2.123 | 8.34× |
| 10 | 10 | 0.538 | 4.350 | 8.09× |
| 100 | 1 | 0.066 | 2.230 | 33.90× |
| 100 | 5 | 0.321 | 12.316 | 38.32× |
| 100 | 10 | 0.592 | 22.745 | 38.43× |
| 1,000 | 1 | 0.172 | 20.766 | 120.78× |
| 1,000 | 5 | 0.401 | 117.186 | 292.04× |
| 1,000 | 10 | 0.685 | 223.320 | 326.19× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.338 | 0.264 | 3.78M | 2197398.692 | 8310963.98× |
| 100,000 | 10 | 2.866 | 1.742 | 5.74M | 2206307.469 | 1266240.02× |
| 100,000 | 1,000 | 99.036 | 93.216 | 10.73M | 2007286.059 | 21533.71× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.77M | 11.14M | 1.00× | 2.38M | 2.46M | 1.00× | 52.20K |
| 5 | 42.81M | 40.64M | 3.65× | 1.98M | 2.05M | 0.83× | 48.61K |
| 10 | 65.44M | 70.93M | 6.36× | 1.95M | 1.92M | 0.78× | 48.58K |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

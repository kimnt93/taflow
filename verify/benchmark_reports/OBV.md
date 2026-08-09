# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.39M | 0.005 | 206.51M | 0.029 | 4.69× | 5.93× |
| 10,000 | 0.041 | 243.38M | 0.038 | 266.41M | 0.060 | 1.47× | 1.61× |
| 100,000 | 0.414 | 241.62M | 0.381 | 262.35M | 0.380 | 0.92× | 1.00× |
| 1,000,000 | 4.411 | 226.68M | 4.016 | 248.98M | 3.509 | 0.80× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.151 | 1.12× |
| 1 | 5 | 0.426 | 0.506 | 1.19× |
| 1 | 10 | 0.488 | 0.958 | 1.96× |
| 10 | 1 | 0.051 | 0.091 | 1.78× |
| 10 | 5 | 0.224 | 0.422 | 1.89× |
| 10 | 10 | 0.444 | 0.894 | 2.01× |
| 100 | 1 | 0.047 | 0.091 | 1.92× |
| 100 | 5 | 0.228 | 0.430 | 1.89× |
| 100 | 10 | 0.472 | 0.890 | 1.89× |
| 1,000 | 1 | 0.052 | 0.095 | 1.82× |
| 1,000 | 5 | 0.229 | 0.446 | 1.95× |
| 1,000 | 10 | 0.496 | 0.939 | 1.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.292 | 0.191 | 5.22M | 369.196 | 1928.66× | 139.78× |
| 100,000 | 10 | 1.542 | 0.752 | 13.30M | 368.642 | 490.36× | 36.50× |
| 100,000 | 1,000 | 6.893 | 5.430 | 184.16M | 377.372 | 69.50× | 5.44× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 157.14M | 174.28M | 1.00× | 2.48M | 3.17M | 1.00× | 190.20M |
| 5 | 520.19M | 668.28M | 3.83× | 2.38M | 2.70M | 0.85× | 205.27M |
| 10 | 616.06M | 965.33M | 5.54× | 2.33M | 2.80M | 0.88× | 212.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

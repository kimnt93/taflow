# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 212.95M | 0.003 | 316.09M | 0.030 | 6.33× | 9.40× |
| 10,000 | 0.010 | 984.53M | 0.007 | 1.46G | 0.033 | 3.22× | 4.77× |
| 100,000 | 0.065 | 1.55G | 0.040 | 2.48G | 0.066 | 1.03× | 1.65× |
| 1,000,000 | 1.160 | 862.24M | 0.756 | 1.32G | 0.801 | 0.69× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.145 | 1.46× |
| 1 | 5 | 0.429 | 0.480 | 1.12× |
| 1 | 10 | 0.480 | 0.908 | 1.89× |
| 10 | 1 | 0.050 | 0.094 | 1.86× |
| 10 | 5 | 0.227 | 0.433 | 1.91× |
| 10 | 10 | 0.515 | 0.941 | 1.83× |
| 100 | 1 | 0.050 | 0.090 | 1.80× |
| 100 | 5 | 0.223 | 0.410 | 1.84× |
| 100 | 10 | 0.488 | 0.918 | 1.88× |
| 1,000 | 1 | 0.049 | 0.101 | 2.08× |
| 1,000 | 5 | 0.242 | 0.454 | 1.88× |
| 1,000 | 10 | 0.482 | 0.934 | 1.94× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.343 | 0.198 | 5.05M | 69.893 | 353.01× | 142.44× |
| 100,000 | 10 | 1.467 | 0.710 | 14.08M | 67.894 | 95.60× | 40.08× |
| 100,000 | 1,000 | 4.006 | 2.048 | 488.38M | 67.992 | 33.21× | 14.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 446.52M | 903.37M | 1.00× | 2.51M | 3.19M | 1.00× | 554.60M |
| 5 | 850.50M | 1.95G | 2.16× | 2.34M | 3.21M | 1.01× | 617.50M |
| 10 | 738.91M | 1.49G | 1.65× | 2.10M | 2.83M | 0.89× | 614.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

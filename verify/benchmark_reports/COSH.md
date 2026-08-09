# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.56M | 0.008 | 120.59M | 0.036 | 3.81× | 4.39× |
| 10,000 | 0.063 | 158.80M | 0.059 | 170.21M | 0.090 | 1.43× | 1.53× |
| 100,000 | 0.595 | 168.04M | 0.561 | 178.31M | 0.608 | 1.02× | 1.08× |
| 1,000,000 | 6.204 | 161.20M | 5.615 | 178.10M | 6.317 | 1.02× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.127 | 0.80× |
| 1 | 5 | 0.249 | 0.491 | 1.97× |
| 1 | 10 | 0.502 | 1.039 | 2.07× |
| 10 | 1 | 0.076 | 0.093 | 1.22× |
| 10 | 5 | 0.221 | 0.439 | 1.99× |
| 10 | 10 | 0.493 | 0.998 | 2.03× |
| 100 | 1 | 0.056 | 0.098 | 1.76× |
| 100 | 5 | 0.257 | 0.462 | 1.80× |
| 100 | 10 | 0.501 | 0.965 | 1.93× |
| 1,000 | 1 | 0.060 | 0.124 | 2.06× |
| 1,000 | 5 | 0.254 | 0.512 | 2.02× |
| 1,000 | 10 | 0.489 | 0.980 | 2.01× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.379 | 0.170 | 5.88M | 601.897 | 3537.45× | 171.77× |
| 100,000 | 10 | 1.211 | 0.634 | 15.76M | 614.201 | 968.21× | 45.71× |
| 100,000 | 1,000 | 9.623 | 7.028 | 142.29M | 653.844 | 93.04× | 5.48× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 101.03M | 139.84M | 1.00× | 2.11M | 2.66M | 1.00× | 120.29M |
| 5 | 243.43M | 328.40M | 2.35× | 1.45M | 2.20M | 0.83× | 77.14M |
| 10 | 283.20M | 283.02M | 2.02× | 1.63M | 2.05M | 0.77× | 134.59M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

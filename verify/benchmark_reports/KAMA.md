# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.00M | 0.006 | 165.07M | 0.039 | 5.88× | 6.52× |
| 10,000 | 0.041 | 243.13M | 0.038 | 266.51M | 0.070 | 1.70× | 1.86× |
| 100,000 | 0.363 | 275.86M | 0.333 | 300.74M | 0.357 | 0.99× | 1.08× |
| 1,000,000 | 3.973 | 251.67M | 3.567 | 280.33M | 3.270 | 0.82× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.113 | 1.19× |
| 1 | 5 | 0.267 | 0.499 | 1.87× |
| 1 | 10 | 0.540 | 1.071 | 1.98× |
| 10 | 1 | 0.054 | 0.094 | 1.74× |
| 10 | 5 | 0.229 | 0.464 | 2.03× |
| 10 | 10 | 0.542 | 1.041 | 1.92× |
| 100 | 1 | 0.063 | 0.111 | 1.76× |
| 100 | 5 | 0.226 | 0.458 | 2.02× |
| 100 | 10 | 0.492 | 0.927 | 1.88× |
| 1,000 | 1 | 0.066 | 0.125 | 1.89× |
| 1,000 | 5 | 0.269 | 0.519 | 1.93× |
| 1,000 | 10 | 0.477 | 0.971 | 2.03× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.282 | 0.197 | 5.07M | 349.248 | 1770.60× | 167.33× |
| 100,000 | 10 | 1.629 | 0.921 | 10.86M | 340.466 | 369.70× | 35.08× |
| 100,000 | 1,000 | 34.339 | 29.038 | 34.44M | 341.648 | 11.77× | 1.27× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 204.99M | 223.85M | 1.00× | 2.55M | 2.71M | 1.00× | 213.87M |
| 5 | 562.61M | 728.61M | 3.25× | 2.95M | 3.18M | 1.17× | 229.42M |
| 10 | 659.66M | 1.02G | 4.55× | 2.79M | 3.14M | 1.16× | 238.46M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

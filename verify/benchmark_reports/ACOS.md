# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.81M | 0.008 | 119.63M | 0.037 | 3.70× | 4.39× |
| 10,000 | 0.073 | 137.89M | 0.075 | 132.75M | 0.096 | 1.33× | 1.28× |
| 100,000 | 0.680 | 147.04M | 0.668 | 149.73M | 0.688 | 1.01× | 1.03× |
| 1,000,000 | 7.358 | 135.91M | 6.705 | 149.14M | 6.734 | 0.92× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.104 | 0.98× |
| 1 | 5 | 0.347 | 0.524 | 1.51× |
| 1 | 10 | 0.456 | 0.892 | 1.96× |
| 10 | 1 | 0.054 | 0.087 | 1.62× |
| 10 | 5 | 0.237 | 0.422 | 1.78× |
| 10 | 10 | 0.487 | 1.016 | 2.08× |
| 100 | 1 | 0.054 | 0.095 | 1.78× |
| 100 | 5 | 0.244 | 0.446 | 1.83× |
| 100 | 10 | 0.629 | 1.127 | 1.79× |
| 1,000 | 1 | 0.073 | 0.113 | 1.55× |
| 1,000 | 5 | 0.262 | 0.508 | 1.94× |
| 1,000 | 10 | 0.549 | 0.977 | 1.78× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.324 | 0.168 | 5.95M | 716.037 | 4258.77× | 166.28× |
| 100,000 | 10 | 1.076 | 0.683 | 14.64M | 685.879 | 1003.87× | 39.18× |
| 100,000 | 1,000 | 9.516 | 8.343 | 119.87M | 683.535 | 81.93× | 4.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.86M | 118.35M | 1.00× | 2.41M | 3.74M | 1.00× | 114.30M |
| 5 | 396.17M | 507.97M | 4.29× | 2.15M | 2.85M | 0.76× | 118.55M |
| 10 | 443.17M | 578.81M | 4.89× | 1.82M | 2.51M | 0.67× | 122.41M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

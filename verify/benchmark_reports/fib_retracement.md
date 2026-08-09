# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 33.97M | 0.026 | 38.99M | 0.774 | 26.28× | 30.17× |
| 10,000 | 0.308 | 32.51M | 0.286 | 34.95M | 1.184 | 3.85× | 4.14× |
| 100,000 | 3.232 | 30.94M | 2.860 | 34.96M | 5.176 | 1.60× | 1.81× |
| 1,000,000 | 59.759 | 16.73M | 38.936 | 25.68M | 64.704 | 1.08× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.901 | 7.47× |
| 1 | 5 | 0.374 | 4.160 | 11.13× |
| 1 | 10 | 0.512 | 8.211 | 16.03× |
| 10 | 1 | 0.066 | 0.847 | 12.77× |
| 10 | 5 | 0.251 | 4.125 | 16.42× |
| 10 | 10 | 0.523 | 8.243 | 15.77× |
| 100 | 1 | 0.058 | 0.822 | 14.20× |
| 100 | 5 | 0.257 | 4.146 | 16.16× |
| 100 | 10 | 0.535 | 8.526 | 15.94× |
| 1,000 | 1 | 0.090 | 0.901 | 10.00× |
| 1,000 | 5 | 0.275 | 4.657 | 16.92× |
| 1,000 | 10 | 0.591 | 9.489 | 16.06× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full |
|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.593 | 0.613 | 1.63M | 5302.719 | 8651.90× |
| 100,000 | 10 | 4.090 | 3.829 | 2.61M | 5103.086 | 1332.90× |
| 100,000 | 1,000 | 279.979 | 327.714 | 3.05M | 5227.036 | 15.95× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22.49M | 32.08M | 1.00× | 1.09M | 724.89K | 1.00× | 17.65M |
| 5 | 47.69M | 64.94M | 2.02× | 707.21K | 836.14K | 1.15× | 39.57M |
| 10 | 72.19M | 124.17M | 3.87× | 844.88K | 704.89K | 0.97× | 38.00M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

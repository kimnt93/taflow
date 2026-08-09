# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.26M | 0.005 | 195.08M | 0.039 | 6.43× | 7.64× |
| 10,000 | 0.034 | 297.88M | 0.029 | 340.61M | 0.065 | 1.94× | 2.22× |
| 100,000 | 0.294 | 339.67M | 0.261 | 382.53M | 0.326 | 1.11× | 1.25× |
| 1,000,000 | 4.097 | 244.08M | 4.250 | 235.28M | 3.046 | 0.74× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.146 | 1.67× |
| 1 | 5 | 0.283 | 0.503 | 1.78× |
| 1 | 10 | 0.517 | 1.051 | 2.03× |
| 10 | 1 | 0.052 | 0.096 | 1.83× |
| 10 | 5 | 0.229 | 0.449 | 1.96× |
| 10 | 10 | 0.554 | 1.065 | 1.92× |
| 100 | 1 | 0.051 | 0.098 | 1.93× |
| 100 | 5 | 0.233 | 0.442 | 1.90× |
| 100 | 10 | 0.518 | 1.067 | 2.06× |
| 1,000 | 1 | 0.059 | 0.111 | 1.89× |
| 1,000 | 5 | 0.236 | 0.481 | 2.04× |
| 1,000 | 10 | 0.491 | 1.027 | 2.09× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.266 | 0.193 | 5.18M | 321.198 | 1664.81× | 169.26× |
| 100,000 | 10 | 1.221 | 0.725 | 13.79M | 322.927 | 445.45× | 45.83× |
| 100,000 | 1,000 | 30.686 | 29.140 | 34.32M | 320.239 | 10.99× | 1.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 154.80M | 220.63M | 1.00× | 2.76M | 2.84M | 1.00× | 214.25M |
| 5 | 357.07M | 412.73M | 1.87× | 2.82M | 2.86M | 1.01× | 230.24M |
| 10 | 335.43M | 467.61M | 2.12× | 2.89M | 2.98M | 1.05× | 238.24M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

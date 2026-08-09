# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.17M | 0.004 | 264.12M | 0.032 | 6.85× | 8.57× |
| 10,000 | 0.021 | 481.90M | 0.018 | 563.29M | 0.040 | 1.95× | 2.28× |
| 100,000 | 0.184 | 543.91M | 0.151 | 664.19M | 0.128 | 0.70× | 0.85× |
| 1,000,000 | 2.113 | 473.28M | 1.613 | 619.97M | 1.027 | 0.49× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.128 | 1.27× |
| 1 | 5 | 0.303 | 0.468 | 1.55× |
| 1 | 10 | 0.457 | 0.950 | 2.08× |
| 10 | 1 | 0.050 | 0.097 | 1.92× |
| 10 | 5 | 0.232 | 0.445 | 1.92× |
| 10 | 10 | 0.448 | 0.916 | 2.04× |
| 100 | 1 | 0.048 | 0.092 | 1.92× |
| 100 | 5 | 0.215 | 0.437 | 2.03× |
| 100 | 10 | 0.454 | 0.922 | 2.03× |
| 1,000 | 1 | 0.051 | 0.088 | 1.73× |
| 1,000 | 5 | 0.241 | 0.456 | 1.89× |
| 1,000 | 10 | 0.498 | 0.961 | 1.93× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.156 | 6.41M | 127.208 | 814.89× | 192.68× |
| 100,000 | 10 | 0.894 | 0.480 | 20.82M | 119.850 | 249.52× | 64.13× |
| 100,000 | 1,000 | 5.059 | 6.258 | 159.80M | 121.726 | 19.45× | 5.08× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 301.70M | 392.37M | 1.00× | 3.97M | 4.54M | 1.00× | 474.02M |
| 5 | 697.78M | 1.29G | 3.29× | 3.16M | 2.94M | 0.65× | 464.92M |
| 10 | 692.99M | 1.22G | 3.10× | 2.71M | 3.11M | 0.69× | 496.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.84M | 0.014 | 72.60M | 0.059 | 3.89× | 4.29× |
| 10,000 | 0.112 | 89.59M | 0.101 | 99.30M | 0.121 | 1.08× | 1.20× |
| 100,000 | 1.063 | 94.04M | 0.982 | 101.80M | 0.743 | 0.70× | 0.76× |
| 1,000,000 | 20.417 | 48.98M | 10.256 | 97.51M | 12.932 | 0.63× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.156 | 2.38× |
| 1 | 5 | 0.273 | 0.620 | 2.27× |
| 1 | 10 | 0.585 | 1.293 | 2.21× |
| 10 | 1 | 0.057 | 0.123 | 2.17× |
| 10 | 5 | 0.254 | 0.541 | 2.13× |
| 10 | 10 | 0.569 | 1.200 | 2.11× |
| 100 | 1 | 0.073 | 0.127 | 1.74× |
| 100 | 5 | 0.256 | 0.576 | 2.25× |
| 100 | 10 | 0.518 | 1.128 | 2.18× |
| 1,000 | 1 | 0.070 | 0.125 | 1.79× |
| 1,000 | 5 | 0.267 | 0.618 | 2.31× |
| 1,000 | 10 | 0.534 | 1.226 | 2.30× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.393 | 0.343 | 2.91M | 852.231 | 2482.86× | 162.07× |
| 100,000 | 10 | 2.518 | 1.491 | 6.71M | 740.859 | 496.90× | 35.68× |
| 100,000 | 1,000 | 107.613 | 100.312 | 9.97M | 765.335 | 7.63× | 0.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 55.80M | 90.71M | 1.00× | 1.69M | 1.69M | 1.00× | 94.83M |
| 5 | 143.70M | 297.97M | 3.28× | 1.50M | 1.02M | 0.60× | 99.46M |
| 10 | 162.40M | 380.05M | 4.19× | 1.19M | 1.26M | 0.75× | 98.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

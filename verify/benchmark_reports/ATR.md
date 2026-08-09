# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.74M | 0.009 | 111.80M | 0.041 | 3.90× | 4.56× |
| 10,000 | 0.065 | 154.07M | 0.058 | 173.43M | 0.093 | 1.43× | 1.61× |
| 100,000 | 0.549 | 182.19M | 0.521 | 191.93M | 0.620 | 1.13× | 1.19× |
| 1,000,000 | 6.244 | 160.15M | 5.465 | 182.99M | 6.671 | 1.07× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.281 | 2.44× |
| 1 | 5 | 0.323 | 0.560 | 1.73× |
| 1 | 10 | 0.507 | 0.963 | 1.90× |
| 10 | 1 | 0.050 | 0.091 | 1.82× |
| 10 | 5 | 0.221 | 0.449 | 2.03× |
| 10 | 10 | 0.464 | 0.923 | 1.99× |
| 100 | 1 | 0.051 | 0.107 | 2.09× |
| 100 | 5 | 0.231 | 0.491 | 2.13× |
| 100 | 10 | 0.475 | 0.928 | 1.96× |
| 1,000 | 1 | 0.061 | 0.102 | 1.66× |
| 1,000 | 5 | 0.242 | 0.477 | 1.97× |
| 1,000 | 10 | 0.510 | 1.029 | 2.02× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.342 | 0.252 | 3.97M | 607.899 | 2414.17× | 132.15× |
| 100,000 | 10 | 2.537 | 0.944 | 10.59M | 625.443 | 662.37× | 35.08× |
| 100,000 | 1,000 | 13.960 | 7.119 | 140.47M | 613.221 | 86.14× | 5.44× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 135.01M | 157.40M | 1.00× | 2.20M | 3.10M | 1.00× | 130.57M |
| 5 | 337.19M | 361.27M | 2.30× | 2.21M | 2.52M | 0.81× | 123.47M |
| 10 | 485.70M | 685.27M | 4.35× | 1.89M | 2.30M | 0.74× | 125.02M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

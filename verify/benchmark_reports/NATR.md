# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.55M | 0.009 | 106.01M | 0.040 | 3.62× | 4.29× |
| 10,000 | 0.064 | 156.31M | 0.060 | 166.92M | 0.091 | 1.42× | 1.52× |
| 100,000 | 0.572 | 174.79M | 0.555 | 180.30M | 0.620 | 1.08× | 1.12× |
| 1,000,000 | 6.123 | 163.32M | 5.843 | 171.16M | 6.663 | 1.09× | 1.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.121 | 0.99× |
| 1 | 5 | 0.353 | 0.573 | 1.62× |
| 1 | 10 | 0.511 | 1.013 | 1.98× |
| 10 | 1 | 0.051 | 0.094 | 1.83× |
| 10 | 5 | 0.227 | 0.506 | 2.22× |
| 10 | 10 | 0.473 | 0.940 | 1.99× |
| 100 | 1 | 0.061 | 0.095 | 1.57× |
| 100 | 5 | 0.234 | 0.453 | 1.93× |
| 100 | 10 | 0.502 | 1.009 | 2.01× |
| 1,000 | 1 | 0.060 | 0.105 | 1.76× |
| 1,000 | 5 | 0.263 | 0.497 | 1.89× |
| 1,000 | 10 | 0.596 | 1.084 | 1.82× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.345 | 0.234 | 4.27M | 601.010 | 2563.71× | 136.10× |
| 100,000 | 10 | 1.845 | 1.181 | 8.47M | 626.887 | 530.73× | 28.43× |
| 100,000 | 1,000 | 11.973 | 8.057 | 124.11M | 606.868 | 75.32× | 4.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.29M | 132.19M | 1.00× | 2.53M | 3.19M | 1.00× | 135.18M |
| 5 | 441.75M | 433.75M | 3.28× | 2.09M | 2.42M | 0.76× | 124.25M |
| 10 | 499.79M | 711.54M | 5.38× | 1.90M | 2.35M | 0.74× | 130.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

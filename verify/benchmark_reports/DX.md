# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.60M | 0.012 | 80.72M | 0.042 | 3.17× | 3.39× |
| 10,000 | 0.119 | 83.97M | 0.117 | 85.64M | 0.115 | 0.97× | 0.99× |
| 100,000 | 1.175 | 85.09M | 1.109 | 90.21M | 0.862 | 0.73× | 0.78× |
| 1,000,000 | 12.686 | 78.83M | 11.767 | 84.98M | 8.674 | 0.68× | 0.74× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.128 ms**; native kernel **1.152 ms**; TA-Lib 0.868 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.287 | 0.226 | 4.42M | 852.150 | 3769.25× | 136.25× |
| 100,000 | 10 | 1.115 | 1.077 | 9.28M | 899.028 | 834.52× | 29.75× |
| 100,000 | 1,000 | 14.160 | 13.034 | 76.72M | 844.796 | 64.82× | 3.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.46M | 78.00M | 1.00× | 2.43M | 2.52M | 1.00× | 94.39M |
| 2 | 135.38M | 150.82M | 1.93× | 2.42M | 2.77M | 1.10× | 97.05M |
| 4 | 246.01M | 283.41M | 3.63× | 2.39M | 2.36M | 0.94× | 95.59M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

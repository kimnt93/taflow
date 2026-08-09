# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.42M | 0.011 | 92.79M | 0.038 | 3.40× | 3.56× |
| 10,000 | 0.111 | 89.70M | 0.100 | 100.20M | 0.115 | 1.03× | 1.15× |
| 100,000 | 0.987 | 101.31M | 0.962 | 104.00M | 0.869 | 0.88× | 0.90× |
| 1,000,000 | 10.215 | 97.89M | 10.120 | 98.82M | 8.894 | 0.87× | 0.88× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.986 ms**; native kernel **0.976 ms**; TA-Lib 0.857 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.281 | 0.230 | 4.35M | 851.460 | 3701.77× | 131.62× |
| 100,000 | 10 | 1.070 | 1.162 | 8.61M | 854.476 | 735.29× | 27.06× |
| 100,000 | 1,000 | 11.886 | 11.248 | 88.90M | 856.630 | 76.16× | 3.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 78.54M | 93.09M | 1.00× | 2.12M | 2.76M | 1.00× | 94.60M |
| 2 | 165.57M | 183.05M | 1.97× | 2.50M | 2.80M | 1.02× | 97.11M |
| 4 | 267.59M | 325.43M | 3.50× | 2.39M | 2.35M | 0.85× | 96.54M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

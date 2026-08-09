# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 190.47M | 0.003 | 293.07M | 0.034 | 6.44× | 9.91× |
| 10,000 | 0.046 | 215.27M | 0.044 | 229.17M | 0.088 | 1.89× | 2.01× |
| 100,000 | 0.520 | 192.46M | 0.506 | 197.58M | 0.636 | 1.22× | 1.26× |
| 1,000,000 | 5.615 | 178.10M | 5.588 | 178.97M | 6.403 | 1.14× | 1.15× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.519 ms**; native kernel **0.502 ms**; TA-Lib 0.637 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.325 | 0.254 | 3.94M | 625.240 | 2463.90× | 111.57× |
| 100,000 | 10 | 2.490 | 1.293 | 7.74M | 632.718 | 489.47× | 22.75× |
| 100,000 | 1,000 | 22.864 | 17.423 | 57.39M | 637.995 | 36.62× | 1.94× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 141.14M | 161.07M | 1.00× | 2.49M | 2.58M | 1.00× | 125.48M |
| 2 | 295.16M | 299.85M | 1.86× | 2.44M | 2.66M | 1.03× | 124.37M |
| 4 | 512.46M | 550.57M | 3.42× | 2.33M | 2.56M | 0.99× | 121.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

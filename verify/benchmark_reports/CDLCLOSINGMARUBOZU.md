# CandleClosingMarubozu benchmark (`CDLCLOSINGMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.42M | 0.005 | 213.92M | 0.035 | 5.34× | 7.40× |
| 10,000 | 0.093 | 107.41M | 0.089 | 112.30M | 0.126 | 1.36× | 1.42× |
| 100,000 | 0.983 | 101.69M | 0.966 | 103.54M | 0.988 | 1.00× | 1.02× |
| 1,000,000 | 10.440 | 95.79M | 10.097 | 99.04M | 10.005 | 0.96× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.979 ms**; native kernel **0.977 ms**; TA-Lib 0.988 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.337 | 0.280 | 3.57M | 999.685 | 3570.71× | 97.40× |
| 100,000 | 10 | 2.512 | 1.293 | 7.73M | 1001.312 | 774.35× | 20.50× |
| 100,000 | 1,000 | 28.427 | 26.657 | 37.51M | 993.089 | 37.25× | 1.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 84.18M | 91.20M | 1.00× | 1.97M | 2.21M | 1.00× | 83.12M |
| 2 | 162.60M | 174.76M | 1.92× | 2.31M | 2.52M | 1.14× | 87.22M |
| 4 | 289.32M | 307.18M | 3.37× | 2.19M | 2.42M | 1.10× | 85.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

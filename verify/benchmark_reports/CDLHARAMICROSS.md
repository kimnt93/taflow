# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.02M | 0.005 | 207.75M | 0.035 | 5.23× | 7.25× |
| 10,000 | 0.088 | 113.93M | 0.084 | 119.55M | 0.135 | 1.54× | 1.61× |
| 100,000 | 1.066 | 93.83M | 1.045 | 95.67M | 1.103 | 1.03× | 1.06× |
| 1,000,000 | 10.936 | 91.44M | 10.829 | 92.35M | 10.765 | 0.98× | 0.99× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.061 ms**; native kernel **1.048 ms**; TA-Lib 1.093 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.340 | 0.282 | 3.54M | 1102.257 | 3901.82× | 96.22× |
| 100,000 | 10 | 2.560 | 1.359 | 7.36M | 1099.764 | 809.11× | 19.80× |
| 100,000 | 1,000 | 29.333 | 31.709 | 31.54M | 1107.449 | 34.93× | 1.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 81.62M | 84.52M | 1.00× | 2.28M | 2.63M | 1.00× | 79.23M |
| 2 | 151.64M | 169.19M | 2.00× | 2.23M | 2.61M | 1.00× | 82.12M |
| 4 | 288.29M | 299.41M | 3.54× | 2.32M | 2.58M | 0.98× | 78.82M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

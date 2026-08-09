# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.43M | 0.014 | 73.33M | 0.038 | 2.62× | 2.77× |
| 10,000 | 0.105 | 95.01M | 0.094 | 106.50M | 0.096 | 0.91× | 1.02× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.019 ms**; native kernel **0.018 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.299 | 0.231 | 4.33M | 40.645 | 176.13× | 138.23× |
| 1,500 | 10 | 2.114 | 1.238 | 8.08M | 40.248 | 32.50× | 25.25× |
| 1,500 | 100 | 4.720 | 3.077 | 32.50M | 41.661 | 13.54× | 10.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.91M | 11.94M | 1.00× | 1.02M | 1.39M | 1.00× | 9.38M |
| 2 | 15.38M | 19.90M | 1.67× | 1.25M | 1.38M | 0.99× | 9.28M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

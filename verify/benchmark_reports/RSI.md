# RelativeStrengthIndex benchmark (`RSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.75M | 0.006 | 156.45M | 0.036 | 4.66× | 5.58× |
| 10,000 | 0.061 | 163.76M | 0.058 | 171.26M | 0.086 | 1.41× | 1.48× |
| 100,000 | 0.597 | 167.62M | 0.563 | 177.50M | 0.591 | 0.99× | 1.05× |
| 1,000,000 | 6.193 | 161.48M | 6.114 | 163.56M | 5.966 | 0.96× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.611 ms**; native kernel **0.589 ms**; TA-Lib 0.600 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.162 | 6.17M | 632.861 | 3902.43× | 203.65× |
| 100,000 | 10 | 0.994 | 0.620 | 16.13M | 603.397 | 973.43× | 51.66× |
| 100,000 | 1,000 | 9.399 | 7.201 | 138.87M | 606.782 | 84.26× | 5.08× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 123.75M | 140.90M | 1.00× | 3.15M | 3.16M | 1.00× | 137.58M |
| 2 | 255.15M | 281.31M | 2.00× | 2.92M | 3.65M | 1.15× | 136.34M |
| 4 | 354.71M | 454.16M | 3.22× | 2.57M | 2.92M | 0.92× | 132.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

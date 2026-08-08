# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.16M | 0.005 | 181.84M | 0.036 | 4.90× | 6.49× |
| 10,000 | 0.049 | 204.73M | 0.045 | 220.20M | 0.108 | 2.20× | 2.37× |
| 100,000 | 0.446 | 224.17M | 0.417 | 239.93M | 0.886 | 1.99× | 2.13× |
| 1,000,000 | 5.517 | 181.25M | 5.047 | 198.14M | 8.718 | 1.58× | 1.73× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.445 ms**; native kernel **0.425 ms**; TA-Lib 0.863 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.438 | 0.292 | 3.43M | 870.130 | 2983.68× | 102.94× |
| 100,000 | 10 | 2.763 | 1.399 | 7.15M | 873.137 | 624.31× | 21.94× |
| 100,000 | 1,000 | 34.538 | 29.126 | 34.33M | 880.575 | 30.23× | 1.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 145.62M | 177.33M | 1.00× | 1.89M | 2.19M | 1.00× | 99.40M |
| 2 | 271.42M | 326.18M | 1.84× | 1.82M | 2.37M | 1.08× | 96.50M |
| 4 | 355.01M | 414.39M | 2.34× | 1.75M | 2.15M | 0.98× | 95.30M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

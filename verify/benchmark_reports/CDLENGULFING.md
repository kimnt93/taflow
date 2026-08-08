# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.10M | 0.003 | 363.41M | 0.030 | 6.60× | 10.99× |
| 10,000 | 0.058 | 172.66M | 0.054 | 184.69M | 0.082 | 1.42× | 1.52× |
| 100,000 | 0.673 | 148.56M | 0.640 | 156.28M | 0.576 | 0.86× | 0.90× |
| 1,000,000 | 6.965 | 143.58M | 6.783 | 147.42M | 5.727 | 0.82× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.686 ms**; native kernel **0.739 ms**; TA-Lib 0.579 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.321 | 0.256 | 3.91M | 571.646 | 2232.51× | 105.71× |
| 100,000 | 10 | 2.826 | 1.483 | 6.74M | 561.388 | 378.43× | 18.36× |
| 100,000 | 1,000 | 23.694 | 21.117 | 47.36M | 583.439 | 27.63× | 1.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 117.61M | 111.19M | 1.00× | 1.89M | 2.10M | 1.00× | 133.11M |
| 2 | 229.29M | 240.10M | 2.16× | 2.49M | 2.71M | 1.29× | 142.56M |
| 4 | 379.55M | 453.55M | 4.08× | 2.32M | 2.46M | 1.17× | 138.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

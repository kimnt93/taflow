# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.72M | 0.005 | 220.11M | 0.036 | 5.64× | 7.98× |
| 10,000 | 0.095 | 105.21M | 0.092 | 108.44M | 0.127 | 1.33× | 1.38× |
| 100,000 | 0.982 | 101.83M | 0.987 | 101.37M | 0.999 | 1.02× | 1.01× |
| 1,000,000 | 10.328 | 96.82M | 10.048 | 99.52M | 10.292 | 1.00× | 1.02× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.985 ms**; native kernel **0.980 ms**; TA-Lib 0.997 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.330 | 0.268 | 3.74M | 1005.359 | 3756.80× | 99.21× |
| 100,000 | 10 | 2.528 | 1.375 | 7.27M | 996.962 | 725.01× | 20.17× |
| 100,000 | 1,000 | 28.386 | 29.032 | 34.44M | 1009.132 | 34.76× | 1.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 87.98M | 86.70M | 1.00× | 2.41M | 2.12M | 1.00× | 90.01M |
| 2 | 163.00M | 176.86M | 2.04× | 2.37M | 2.67M | 1.26× | 86.54M |
| 4 | 301.59M | 318.90M | 3.68× | 2.34M | 2.49M | 1.17× | 87.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

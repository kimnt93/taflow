# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.69M | 0.056 | 17.75M | 0.090 | 1.60× | 1.60× |
| 10,000 | 0.591 | 16.91M | 0.548 | 18.25M | 0.599 | 1.01× | 1.09× |
| 100,000 | 5.806 | 17.22M | 5.542 | 18.04M | 5.188 | 0.89× | 0.94× |
| 1,000,000 | 55.727 | 17.94M | 55.912 | 17.89M | 50.964 | 0.91× | 0.91× |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.553 ms**; native kernel **5.425 ms**; TA-Lib 5.336 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.428 | 0.318 | 3.14M | 5367.706 | 16881.29× | 131.58× |
| 100,000 | 10 | 2.316 | 1.698 | 5.89M | 5200.822 | 3062.10× | 23.04× |
| 100,000 | 1,000 | 127.902 | 101.070 | 9.89M | 5278.152 | 52.22× | 0.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 17.93M | 18.21M | 1.00× | 1.83M | 1.93M | 1.00× | 18.28M |
| 2 | 35.20M | 36.02M | 1.98× | 1.85M | 2.07M | 1.07× | 18.26M |
| 4 | 63.17M | 70.65M | 3.88× | 1.77M | 1.98M | 1.02× | 17.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

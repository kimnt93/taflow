# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 239.11M | 0.002 | 408.65M | 0.029 | 6.93× | 11.84× |
| 10,000 | 0.060 | 166.54M | 0.055 | 181.69M | 0.080 | 1.33× | 1.45× |
| 100,000 | 0.666 | 150.08M | 0.662 | 151.06M | 0.544 | 0.82× | 0.82× |
| 1,000,000 | 6.915 | 144.61M | 6.871 | 145.55M | 5.263 | 0.76× | 0.77× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.678 ms**; native kernel **0.665 ms**; TA-Lib 0.555 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.309 | 0.246 | 4.07M | 547.386 | 2228.73× | 108.85× |
| 100,000 | 10 | 2.509 | 1.261 | 7.93M | 614.111 | 487.09× | 21.22× |
| 100,000 | 1,000 | 21.772 | 21.489 | 46.54M | 567.232 | 26.40× | 1.35× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 116.85M | 118.73M | 1.00× | 2.43M | 2.45M | 1.00× | 134.70M |
| 2 | 235.51M | 232.25M | 1.96× | 2.57M | 2.76M | 1.13× | 140.29M |
| 4 | 370.09M | 410.51M | 3.46× | 2.40M | 2.47M | 1.01× | 138.30M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

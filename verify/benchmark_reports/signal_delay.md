# SignalDelay benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 192.99M | 0.004 | 241.70M | nan | — | — |
| 10,000 | 0.034 | 293.12M | 0.032 | 316.37M | nan | — | — |
| 100,000 | 0.328 | 304.74M | 0.303 | 329.59M | nan | — | — |
| 1,000,000 | 3.520 | 284.10M | 3.120 | 320.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.322 ms**; native kernel **0.302 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.220 | 0.150 | 6.65M | nan | — | — |
| 100,000 | 10 | 0.934 | 0.506 | 19.77M | nan | — | — |
| 100,000 | 1,000 | 4.968 | 4.274 | 234.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 204.68M | 206.49M | 1.00× | 3.42M | 3.17M | 1.00× | — |
| 2 | 377.88M | 408.41M | 1.98× | 3.56M | 4.43M | 1.40× | — |
| 4 | 472.15M | 864.34M | 4.19× | 3.59M | 3.89M | 1.23× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

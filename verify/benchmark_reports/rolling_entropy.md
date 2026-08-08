# RollingEntropy benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.711 | 1.41M | 0.703 | 1.42M | nan | — | — |
| 10,000 | 7.042 | 1.42M | 7.061 | 1.42M | nan | — | — |
| 100,000 | 72.682 | 1.38M | 70.724 | 1.41M | nan | — | — |
| 1,000,000 | 709.339 | 1.41M | 700.334 | 1.43M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **69.829 ms**; native kernel **70.585 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.952 | 0.860 | 1.16M | nan | — | — |
| 100,000 | 10 | 8.290 | 7.964 | 1.26M | nan | — | — |
| 100,000 | 1,000 | 756.097 | 724.311 | 1.38M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.36M | 1.39M | 1.00× | 908.30K | 731.79K | 1.00× | — |
| 2 | 2.54M | 2.56M | 1.84× | 844.92K | 857.69K | 1.17× | — |
| 4 | 5.03M | 4.75M | 3.43× | 915.29K | 892.23K | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

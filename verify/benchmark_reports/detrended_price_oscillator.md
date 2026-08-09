# DetrendedPriceOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.32M | 0.006 | 180.90M | nan | — | — |
| 10,000 | 0.061 | 164.83M | 0.045 | 219.80M | nan | — | — |
| 100,000 | 0.453 | 220.87M | 0.435 | 229.73M | nan | — | — |
| 1,000,000 | 4.917 | 203.37M | 4.336 | 230.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.462 ms**; native kernel **0.425 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.210 | 0.160 | 6.23M | nan | — | — |
| 100,000 | 10 | 0.926 | 0.535 | 18.70M | nan | — | — |
| 100,000 | 1,000 | 8.346 | 5.719 | 174.85M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 158.45M | 159.80M | 1.00× | 3.75M | 3.29M | 1.00× | — |
| 2 | 296.58M | 141.27M | 0.88× | 3.21M | 4.07M | 1.24× | — |
| 4 | 234.10M | 491.20M | 3.07× | 3.71M | 3.66M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

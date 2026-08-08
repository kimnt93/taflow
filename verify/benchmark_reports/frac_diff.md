# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.072 | 13.87M | 0.072 | 13.90M | nan | — | — |
| 10,000 | 7.683 | 1.30M | 7.352 | 1.36M | nan | — | — |
| 100,000 | 83.114 | 1.20M | 81.268 | 1.23M | nan | — | — |
| 1,000,000 | 819.425 | 1.22M | 816.332 | 1.22M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **80.460 ms**; native kernel **82.105 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 1.013 | 0.969 | 1.03M | nan | — | — |
| 100,000 | 10 | 9.044 | 8.662 | 1.15M | nan | — | — |
| 100,000 | 1,000 | 824.108 | 786.722 | 1.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.26M | 1.25M | 1.00× | 787.77K | 843.16K | 1.00× | — |
| 2 | 2.42M | 2.41M | 1.94× | 819.66K | 802.14K | 0.95× | — |
| 4 | 4.64M | 4.66M | 3.74× | 822.36K | 856.12K | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# KeltnerChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.80M | 0.010 | 102.97M | nan | — | — |
| 10,000 | 0.082 | 121.33M | 0.081 | 124.07M | nan | — | — |
| 100,000 | 0.802 | 124.65M | 0.765 | 130.64M | nan | — | — |
| 1,000,000 | 21.179 | 47.22M | 9.194 | 108.77M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.804 ms**; native kernel **0.783 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.308 | 0.250 | 4.00M | nan | — | — |
| 100,000 | 10 | 1.878 | 0.952 | 10.51M | nan | — | — |
| 100,000 | 1,000 | 12.386 | 8.981 | 111.34M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 83.48M | 101.25M | 1.00× | 2.18M | 2.13M | 1.00× | — |
| 2 | 135.32M | 194.35M | 1.92× | 2.46M | 2.62M | 1.23× | — |
| 4 | 165.82M | 290.64M | 2.87× | 2.36M | 2.38M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

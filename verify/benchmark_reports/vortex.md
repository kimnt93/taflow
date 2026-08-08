# Vortex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.29M | 0.018 | 54.49M | nan | — | — |
| 10,000 | 0.161 | 62.08M | 0.161 | 62.06M | nan | — | — |
| 100,000 | 1.555 | 64.31M | 1.571 | 63.64M | nan | — | — |
| 1,000,000 | 17.140 | 58.34M | 16.515 | 60.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.552 ms**; native kernel **1.632 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.355 | 0.281 | 3.56M | nan | — | — |
| 100,000 | 10 | 2.217 | 1.226 | 8.16M | nan | — | — |
| 100,000 | 1,000 | 23.516 | 20.615 | 48.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 48.22M | 52.49M | 1.00× | 2.28M | 2.11M | 1.00× | — |
| 2 | 101.13M | 108.34M | 2.06× | 2.43M | 2.46M | 1.17× | — |
| 4 | 122.65M | 112.49M | 2.14× | 2.34M | 2.41M | 1.15× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

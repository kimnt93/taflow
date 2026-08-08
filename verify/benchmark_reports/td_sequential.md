# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.84M | 0.007 | 153.13M | nan | — | — |
| 10,000 | 0.063 | 158.17M | 0.061 | 162.84M | nan | — | — |
| 100,000 | 0.597 | 167.56M | 0.584 | 171.14M | nan | — | — |
| 1,000,000 | 6.562 | 152.40M | 6.894 | 145.06M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.632 ms**; native kernel **0.582 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.254 | 0.180 | 5.55M | nan | — | — |
| 100,000 | 10 | 0.683 | 0.518 | 19.31M | nan | — | — |
| 100,000 | 1,000 | 8.182 | 7.664 | 130.48M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 133.32M | 152.75M | 1.00× | 2.82M | 3.47M | 1.00× | — |
| 2 | 111.84M | 148.86M | 0.97× | 2.77M | 3.78M | 1.09× | — |
| 4 | 128.09M | 154.71M | 1.01× | 3.22M | 3.50M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

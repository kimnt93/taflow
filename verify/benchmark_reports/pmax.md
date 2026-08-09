# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.09M | 0.025 | 39.71M | nan | — | — |
| 10,000 | 0.206 | 48.54M | 0.199 | 50.26M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.035 ms**; native kernel **0.034 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.456 | 0.358 | 2.80M | nan | — | — |
| 1,500 | 10 | 2.152 | 1.408 | 7.10M | nan | — | — |
| 1,500 | 100 | 4.231 | 3.495 | 28.62M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.61M | 12.56M | 1.00× | 988.90K | 996.79K | 1.00× | — |
| 2 | 13.49M | 13.33M | 1.06× | 1.11M | 1.34M | 1.34× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

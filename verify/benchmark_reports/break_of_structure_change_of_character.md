# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.69M | 0.045 | 22.23M | nan | — | — |
| 10,000 | 0.455 | 21.98M | 0.441 | 22.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.072 ms**; native kernel **0.069 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.417 | 0.339 | 2.95M | nan | — | — |
| 1,500 | 10 | 2.559 | 1.477 | 6.77M | nan | — | — |
| 1,500 | 100 | 7.164 | 6.060 | 16.50M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.36M | 6.02M | 1.00× | 811.06K | 1.12M | 1.00× | — |
| 2 | 11.61M | 13.56M | 2.25× | 932.60K | 1.12M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

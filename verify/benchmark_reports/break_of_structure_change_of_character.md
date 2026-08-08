# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.88M | 0.041 | 24.15M | nan | — | — |
| 10,000 | 0.441 | 22.68M | 0.432 | 23.16M | nan | — | — |
| 100,000 | 5.212 | 19.19M | 4.289 | 23.31M | nan | — | — |
| 1,000,000 | 61.872 | 16.16M | 43.887 | 22.79M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.502 ms**; native kernel **4.433 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.396 | 0.315 | 3.18M | nan | — | — |
| 100,000 | 10 | 2.471 | 1.450 | 6.90M | nan | — | — |
| 100,000 | 1,000 | 54.133 | 48.943 | 20.43M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.31M | 22.45M | 1.00× | 1.72M | 1.63M | 1.00× | — |
| 2 | 35.44M | 42.96M | 1.91× | 2.14M | 2.04M | 1.25× | — |
| 4 | 54.53M | 61.88M | 2.76× | 1.98M | 1.99M | 1.22× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

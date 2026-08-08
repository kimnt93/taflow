# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.69M | 0.006 | 159.19M | nan | — | — |
| 10,000 | 0.061 | 164.82M | 0.054 | 185.41M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.195 | 5.14M | nan | — | — |
| 1,500 | 10 | 0.911 | 0.593 | 16.88M | nan | — | — |
| 1,500 | 100 | 2.622 | 2.223 | 44.98M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

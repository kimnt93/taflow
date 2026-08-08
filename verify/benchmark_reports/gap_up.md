# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.84M | 0.004 | 259.48M | nan | — | — |
| 10,000 | 0.034 | 293.18M | 0.031 | 321.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.320 | 0.228 | 4.38M | nan | — | — |
| 1,500 | 10 | 1.661 | 0.777 | 12.87M | nan | — | — |
| 1,500 | 100 | 3.291 | 2.209 | 45.27M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.22M | 0.010 | 96.93M | nan | — | — |
| 10,000 | 0.098 | 102.12M | 0.094 | 106.11M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.016 ms**; native kernel **0.015 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.294 | 0.196 | 5.09M | nan | — | — |
| 1,500 | 10 | 1.201 | 0.679 | 14.72M | nan | — | — |
| 1,500 | 100 | 3.360 | 2.683 | 37.27M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

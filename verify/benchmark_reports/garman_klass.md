# GarmanKlass benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.17M | 0.018 | 56.45M | nan | — | — |
| 10,000 | 0.166 | 60.26M | 0.162 | 61.86M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.028 ms**; native kernel **0.026 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.419 | 0.307 | 3.25M | nan | — | — |
| 1,500 | 10 | 2.744 | 1.307 | 7.65M | nan | — | — |
| 1,500 | 100 | 5.574 | 4.106 | 24.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

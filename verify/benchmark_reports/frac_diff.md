# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.078 | 12.82M | 0.078 | 12.83M | nan | — | — |
| 10,000 | 8.071 | 1.24M | 8.032 | 1.25M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.516 ms**; native kernel **0.528 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 1.191 | 1.068 | 936.29K | nan | — | — |
| 1,500 | 10 | 10.120 | 13.962 | 716.24K | nan | — | — |
| 1,500 | 100 | 90.112 | 89.385 | 1.12M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

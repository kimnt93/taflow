# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.29M | 0.019 | 53.89M | nan | — | — |
| 10,000 | 0.588 | 17.01M | 0.175 | 57.11M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.089 ms**; native kernel **0.027 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.374 | 0.201 | 4.98M | nan | — | — |
| 1,500 | 10 | 1.775 | 0.783 | 12.76M | nan | — | — |
| 1,500 | 100 | 7.868 | 3.601 | 27.77M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

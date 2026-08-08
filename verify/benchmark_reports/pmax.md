# ParabolicMovingAverageStop benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.59M | 0.020 | 49.34M | nan | — | — |
| 10,000 | 0.190 | 52.55M | 0.187 | 53.59M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.031 ms**; native kernel **0.030 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.432 | 0.298 | 3.35M | nan | — | — |
| 1,500 | 10 | 1.660 | 1.148 | 8.71M | nan | — | — |
| 1,500 | 100 | 4.613 | 3.725 | 26.85M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

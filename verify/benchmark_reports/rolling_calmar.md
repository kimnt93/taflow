# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 36.35M | 0.027 | 37.70M | nan | — | — |
| 10,000 | 0.258 | 38.82M | 0.259 | 38.56M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.039 ms**; native kernel **0.039 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.337 | 0.209 | 4.78M | nan | — | — |
| 1,500 | 10 | 1.325 | 1.199 | 8.34M | nan | — | — |
| 1,500 | 100 | 5.278 | 3.928 | 25.46M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

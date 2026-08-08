# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.88M | 0.007 | 153.53M | nan | — | — |
| 10,000 | 0.060 | 166.63M | 0.057 | 176.57M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.432 | 0.192 | 5.22M | nan | — | — |
| 1,500 | 10 | 1.111 | 0.581 | 17.20M | nan | — | — |
| 1,500 | 100 | 2.904 | 2.209 | 45.27M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

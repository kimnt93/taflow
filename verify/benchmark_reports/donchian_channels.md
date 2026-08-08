# Donchian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.65M | 0.050 | 20.06M | nan | — | — |
| 10,000 | 0.525 | 19.03M | 0.518 | 19.29M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.076 ms**; native kernel **0.074 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.392 | 0.299 | 3.35M | nan | — | — |
| 1,500 | 10 | 2.301 | 3.151 | 3.17M | nan | — | — |
| 1,500 | 100 | 7.980 | 6.513 | 15.35M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

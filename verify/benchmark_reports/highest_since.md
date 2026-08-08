# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.70M | 0.005 | 216.27M | nan | — | — |
| 10,000 | 0.040 | 247.13M | 0.038 | 262.26M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.451 | 0.322 | 3.11M | nan | — | — |
| 1,500 | 10 | 1.420 | 0.781 | 12.80M | nan | — | — |
| 1,500 | 100 | 3.054 | 2.086 | 47.94M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

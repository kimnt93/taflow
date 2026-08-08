# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.48M | 0.105 | 9.53M | nan | — | — |
| 10,000 | 1.039 | 9.63M | 1.055 | 9.47M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.162 ms**; native kernel **0.161 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.420 | 0.316 | 3.16M | nan | — | — |
| 1,500 | 10 | 2.508 | 1.720 | 5.81M | nan | — | — |
| 1,500 | 100 | 13.182 | 11.674 | 8.57M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

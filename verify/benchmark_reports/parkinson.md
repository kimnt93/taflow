# Parkinson benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.17M | 0.015 | 68.52M | nan | — | — |
| 10,000 | 0.138 | 72.40M | 0.134 | 74.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.021 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.325 | 0.229 | 4.37M | nan | — | — |
| 1,500 | 10 | 1.714 | 0.862 | 11.60M | nan | — | — |
| 1,500 | 100 | 4.291 | 3.323 | 30.09M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

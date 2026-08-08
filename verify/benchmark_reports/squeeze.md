# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.83M | 0.038 | 26.08M | nan | — | — |
| 10,000 | 0.383 | 26.09M | 0.366 | 27.33M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.060 ms**; native kernel **0.057 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.454 | 0.344 | 2.91M | nan | — | — |
| 1,500 | 10 | 2.509 | 1.351 | 7.40M | nan | — | — |
| 1,500 | 100 | 7.570 | 5.842 | 17.12M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

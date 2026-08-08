# UlcerIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.54M | 0.047 | 21.39M | nan | — | — |
| 10,000 | 0.484 | 20.66M | 0.483 | 20.70M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.069 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.306 | 0.219 | 4.56M | nan | — | — |
| 1,500 | 10 | 1.436 | 0.983 | 10.17M | nan | — | — |
| 1,500 | 100 | 6.503 | 5.897 | 16.96M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

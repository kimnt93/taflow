# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.17M | 0.008 | 120.83M | nan | — | — |
| 10,000 | 0.075 | 133.86M | 0.071 | 141.13M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.330 | 0.228 | 4.39M | nan | — | — |
| 1,500 | 10 | 1.719 | 0.846 | 11.82M | nan | — | — |
| 1,500 | 100 | 3.738 | 2.667 | 37.50M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

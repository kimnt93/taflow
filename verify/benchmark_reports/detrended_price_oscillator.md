# DetrendedPriceOscillator benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.62M | 0.008 | 124.43M | nan | — | — |
| 10,000 | 0.075 | 133.70M | 0.071 | 140.11M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.294 | 0.196 | 5.10M | nan | — | — |
| 1,500 | 10 | 1.203 | 0.662 | 15.10M | nan | — | — |
| 1,500 | 100 | 3.249 | 2.377 | 42.07M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

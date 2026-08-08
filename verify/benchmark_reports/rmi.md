# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.56M | 0.009 | 114.09M | nan | — | — |
| 10,000 | 0.079 | 126.94M | 0.112 | 88.94M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.302 | 0.187 | 5.33M | nan | — | — |
| 1,500 | 10 | 1.354 | 0.990 | 10.11M | nan | — | — |
| 1,500 | 100 | 3.240 | 2.713 | 36.86M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

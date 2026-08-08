# Sessions benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.90M | 0.009 | 110.29M | nan | — | — |
| 10,000 | 0.086 | 115.88M | 0.078 | 128.23M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.016 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.537 | 0.402 | 2.49M | nan | — | — |
| 1,500 | 10 | 2.040 | 1.062 | 9.41M | nan | — | — |
| 1,500 | 100 | 4.195 | 2.933 | 34.09M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

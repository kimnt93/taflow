# ForceIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 193.77M | 0.004 | 264.80M | nan | — | — |
| 10,000 | 0.035 | 285.19M | 0.032 | 314.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.435 | 0.218 | 4.59M | nan | — | — |
| 1,500 | 10 | 1.603 | 0.770 | 12.99M | nan | — | — |
| 1,500 | 100 | 3.153 | 2.073 | 48.24M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

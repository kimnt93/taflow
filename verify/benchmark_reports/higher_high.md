# HigherHigh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 170.26M | 0.004 | 255.81M | nan | — | — |
| 10,000 | 0.037 | 266.80M | 0.031 | 322.24M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.220 | 4.54M | nan | — | — |
| 1,500 | 10 | 1.610 | 0.770 | 12.98M | nan | — | — |
| 1,500 | 100 | 3.422 | 2.109 | 47.42M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

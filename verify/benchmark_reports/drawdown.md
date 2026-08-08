# Drawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.98M | 0.005 | 188.02M | nan | — | — |
| 10,000 | 0.048 | 208.69M | 0.042 | 236.24M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.259 | 0.172 | 5.80M | nan | — | — |
| 1,500 | 10 | 1.005 | 0.514 | 19.47M | nan | — | — |
| 1,500 | 100 | 2.512 | 1.835 | 54.50M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingEntropy benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.539 | 1.85M | 0.540 | 1.85M | nan | — | — |
| 10,000 | 5.448 | 1.84M | 5.427 | 1.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.806 ms**; native kernel **0.808 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.832 | 0.737 | 1.36M | nan | — | — |
| 1,500 | 10 | 6.507 | 5.835 | 1.71M | nan | — | — |
| 1,500 | 100 | 55.695 | 54.284 | 1.84M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

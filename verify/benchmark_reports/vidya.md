# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.49M | 0.064 | 15.70M | nan | — | — |
| 10,000 | 0.753 | 13.27M | 0.695 | 14.39M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.105 ms**; native kernel **0.103 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.357 | 0.251 | 3.99M | nan | — | — |
| 1,500 | 10 | 1.556 | 1.284 | 7.79M | nan | — | — |
| 1,500 | 100 | 9.739 | 12.161 | 8.22M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

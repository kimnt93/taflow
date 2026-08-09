# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 86.98M | 0.011 | 91.08M | nan | — | — |
| 10,000 | 0.124 | 80.40M | 0.119 | 84.11M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.019 ms**; native kernel **0.017 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.273 | 0.189 | 5.29M | nan | — | — |
| 1,500 | 10 | 0.968 | 0.723 | 13.84M | nan | — | — |
| 1,500 | 100 | 3.097 | 2.728 | 36.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.21M | 15.35M | 1.00× | 1.43M | 1.43M | 1.00× | — |
| 2 | 16.77M | 21.46M | 1.40× | 1.57M | 1.53M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

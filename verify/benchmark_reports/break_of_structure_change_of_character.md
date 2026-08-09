# BreakOfStructureChangeOfCharacter benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.39M | 0.038 | 26.06M | nan | — | — |
| 10,000 | 0.415 | 24.10M | 0.396 | 25.24M | nan | — | — |
| 100,000 | 4.087 | 24.47M | 3.980 | 25.13M | nan | — | — |
| 1,000,000 | 54.229 | 18.44M | 40.753 | 24.54M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.145 ms**; native kernel **4.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.361 | 0.310 | 3.22M | nan | — | — |
| 100,000 | 10 | 2.456 | 1.364 | 7.33M | nan | — | — |
| 100,000 | 1,000 | 42.755 | 40.664 | 24.59M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.86M | 23.08M | 1.00× | 2.09M | 2.34M | 1.00× | — |
| 2 | 38.79M | 44.49M | 1.93× | 2.05M | 1.97M | 0.84× | — |
| 4 | 56.02M | 63.13M | 2.73× | 1.99M | 1.97M | 0.84× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

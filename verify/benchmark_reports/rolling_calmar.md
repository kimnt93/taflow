# RollingCalmar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.74M | 0.047 | 21.23M | nan | — | — |
| 10,000 | 0.449 | 22.27M | 0.456 | 21.91M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.071 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.276 | 0.207 | 4.84M | nan | — | — |
| 1,500 | 10 | 1.352 | 0.950 | 10.52M | nan | — | — |
| 1,500 | 100 | 5.938 | 5.473 | 18.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.49M | 10.46M | 1.00× | 1.45M | 1.36M | 1.00× | — |
| 2 | 13.91M | 12.53M | 1.20× | 1.46M | 1.50M | 1.10× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

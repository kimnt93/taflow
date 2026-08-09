# ForceIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.37M | 0.004 | 279.41M | nan | — | — |
| 10,000 | 0.029 | 345.88M | 0.025 | 396.04M | nan | — | — |
| 100,000 | 0.277 | 361.43M | 0.253 | 395.02M | nan | — | — |
| 1,000,000 | 3.331 | 300.24M | 2.755 | 362.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.287 ms**; native kernel **0.252 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.258 | 0.196 | 5.10M | nan | — | — |
| 100,000 | 10 | 1.481 | 0.943 | 10.61M | nan | — | — |
| 100,000 | 1,000 | 5.537 | 4.298 | 232.69M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 266.67M | 324.11M | 1.00× | 3.36M | 3.39M | 1.00× | — |
| 2 | 401.95M | 546.82M | 1.69× | 3.27M | 3.47M | 1.02× | — |
| 4 | 572.76M | 972.49M | 3.00× | 3.43M | 3.53M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

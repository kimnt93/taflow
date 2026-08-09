# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.34M | 0.007 | 141.47M | nan | — | — |
| 10,000 | 0.067 | 149.49M | 0.064 | 155.38M | nan | — | — |
| 100,000 | 0.654 | 152.81M | 0.638 | 156.67M | nan | — | — |
| 1,000,000 | 6.726 | 148.67M | 6.353 | 157.40M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.667 ms**; native kernel **0.635 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.215 | 0.152 | 6.59M | nan | — | — |
| 100,000 | 10 | 0.714 | 0.521 | 19.21M | nan | — | — |
| 100,000 | 1,000 | 8.303 | 7.649 | 130.73M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 95.05M | 121.22M | 1.00× | 2.84M | 2.94M | 1.00× | — |
| 2 | 235.13M | 245.64M | 2.03× | 3.00M | 3.73M | 1.27× | — |
| 4 | 231.25M | 261.94M | 2.16× | 3.08M | 3.32M | 1.13× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

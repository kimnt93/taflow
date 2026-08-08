# DecayLinear benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 197.77M | 0.005 | 214.94M | nan | — | — |
| 10,000 | 0.034 | 291.72M | 0.038 | 262.95M | nan | — | — |
| 100,000 | 0.336 | 297.57M | 0.374 | 267.40M | nan | — | — |
| 1,000,000 | 3.693 | 270.81M | 3.847 | 259.94M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.332 ms**; native kernel **0.366 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.229 | 0.165 | 6.08M | nan | — | — |
| 100,000 | 10 | 0.959 | 0.609 | 16.41M | nan | — | — |
| 100,000 | 1,000 | 5.224 | 5.253 | 190.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 210.26M | 264.35M | 1.00× | 3.37M | 3.48M | 1.00× | — |
| 2 | 443.26M | 443.02M | 1.68× | 3.05M | 4.21M | 1.21× | — |
| 4 | 546.89M | 569.01M | 2.15× | 2.93M | 3.26M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

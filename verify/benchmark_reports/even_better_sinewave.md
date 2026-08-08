# EvenBetterSinewave benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.12M | 0.007 | 145.93M | nan | — | — |
| 10,000 | 0.057 | 174.03M | 0.053 | 189.66M | nan | — | — |
| 100,000 | 0.542 | 184.49M | 0.511 | 195.60M | nan | — | — |
| 1,000,000 | 6.080 | 164.47M | 5.404 | 185.06M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.542 ms**; native kernel **0.514 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.166 | 6.02M | nan | — | — |
| 100,000 | 10 | 0.697 | 0.516 | 19.40M | nan | — | — |
| 100,000 | 1,000 | 7.493 | 6.655 | 150.27M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 151.70M | 161.84M | 1.00× | 3.24M | 3.88M | 1.00× | — |
| 2 | 167.00M | 177.03M | 1.09× | 3.33M | 3.72M | 0.96× | — |
| 4 | 155.83M | 171.95M | 1.06× | 2.97M | 3.53M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

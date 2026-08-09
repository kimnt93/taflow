# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.63M | 0.006 | 169.56M | nan | — | — |
| 10,000 | 0.059 | 169.37M | 0.059 | 170.23M | nan | — | — |
| 100,000 | 0.564 | 177.39M | 0.559 | 178.85M | nan | — | — |
| 1,000,000 | 6.683 | 149.63M | 5.597 | 178.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.568 ms**; native kernel **0.567 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.165 | 6.06M | nan | — | — |
| 100,000 | 10 | 0.632 | 0.452 | 22.12M | nan | — | — |
| 100,000 | 1,000 | 6.973 | 6.395 | 156.38M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 143.43M | 153.49M | 1.00× | 3.03M | 3.61M | 1.00× | — |
| 2 | 143.10M | 155.92M | 1.02× | 3.46M | 3.73M | 1.03× | — |
| 4 | 141.51M | 149.73M | 0.98× | 3.18M | 3.53M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

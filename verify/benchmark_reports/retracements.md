# Retracements benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.04M | 0.057 | 17.42M | nan | — | — |
| 10,000 | 0.445 | 22.48M | 0.560 | 17.85M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.066 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.412 | 0.340 | 2.94M | nan | — | — |
| 1,500 | 10 | 2.490 | 1.413 | 7.08M | nan | — | — |
| 1,500 | 100 | 7.154 | 5.912 | 16.91M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.72M | 8.17M | 1.00× | 1.09M | 751.92K | 1.00× | — |
| 2 | 13.58M | 14.42M | 1.76× | 1.27M | 1.23M | 1.63× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

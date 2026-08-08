# TrueStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.78M | 0.006 | 161.97M | nan | — | — |
| 10,000 | 0.059 | 170.75M | 0.060 | 165.73M | nan | — | — |
| 100,000 | 0.492 | 203.09M | 0.480 | 208.21M | nan | — | — |
| 1,000,000 | 5.515 | 181.34M | 5.279 | 189.44M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.555 ms**; native kernel **0.468 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.164 | 6.10M | nan | — | — |
| 100,000 | 10 | 0.961 | 0.568 | 17.61M | nan | — | — |
| 100,000 | 1,000 | 6.821 | 6.016 | 166.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 172.90M | 215.26M | 1.00× | 3.89M | 3.45M | 1.00× | — |
| 2 | 298.26M | 371.93M | 1.73× | 3.36M | 3.45M | 1.00× | — |
| 4 | 411.51M | 659.27M | 3.06× | 3.45M | 3.83M | 1.11× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

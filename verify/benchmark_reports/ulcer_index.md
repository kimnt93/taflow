# UlcerIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.79M | 0.050 | 19.90M | nan | — | — |
| 10,000 | 0.460 | 21.74M | 0.439 | 22.80M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.067 ms**; native kernel **0.068 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.439 | 0.238 | 4.21M | nan | — | — |
| 1,500 | 10 | 1.682 | 1.019 | 9.81M | nan | — | — |
| 1,500 | 100 | 5.626 | 5.202 | 19.22M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.45M | 9.99M | 1.00× | 973.03K | 931.54K | 1.00× | — |
| 2 | 13.42M | 13.92M | 1.39× | 1.52M | 1.37M | 1.47× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

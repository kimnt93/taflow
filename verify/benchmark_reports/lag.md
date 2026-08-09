# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.45M | 0.006 | 178.36M | nan | — | — |
| 10,000 | 0.036 | 274.77M | 0.032 | 310.90M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.007 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.438 | 0.210 | 4.76M | nan | — | — |
| 1,500 | 10 | 0.988 | 0.558 | 17.91M | nan | — | — |
| 1,500 | 100 | 2.052 | 1.485 | 67.36M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.87M | 16.36M | 1.00× | 1.10M | 1.49M | 1.00× | — |
| 2 | 19.61M | 23.21M | 1.42× | 1.57M | 1.74M | 1.17× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

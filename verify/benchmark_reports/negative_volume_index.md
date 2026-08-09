# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.91M | 0.007 | 147.64M | nan | — | — |
| 10,000 | 0.064 | 156.87M | 0.059 | 168.08M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.010 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.281 | 0.209 | 4.79M | nan | — | — |
| 1,500 | 10 | 1.556 | 0.835 | 11.97M | nan | — | — |
| 1,500 | 100 | 2.920 | 2.118 | 47.21M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.32M | 14.25M | 1.00× | 1.34M | 873.00K | 1.00× | — |
| 2 | 14.91M | 20.16M | 1.41× | 1.35M | 1.38M | 1.58× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

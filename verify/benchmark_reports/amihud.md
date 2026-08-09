# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.02M | 0.008 | 122.65M | nan | — | — |
| 10,000 | 0.069 | 144.80M | 0.068 | 146.49M | nan | — | — |
| 100,000 | 0.656 | 152.53M | 0.629 | 159.06M | nan | — | — |
| 1,000,000 | 7.031 | 142.22M | 6.667 | 149.99M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.662 ms**; native kernel **0.630 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.184 | 5.44M | nan | — | — |
| 100,000 | 10 | 1.490 | 0.745 | 13.42M | nan | — | — |
| 100,000 | 1,000 | 8.884 | 7.702 | 129.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 119.44M | 127.37M | 1.00× | 3.29M | 3.58M | 1.00× | — |
| 2 | 226.01M | 253.22M | 1.99× | 3.09M | 3.28M | 0.92× | — |
| 4 | 312.13M | 482.42M | 3.79× | 3.10M | 3.20M | 0.89× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

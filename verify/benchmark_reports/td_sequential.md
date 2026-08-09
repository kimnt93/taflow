# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.23M | 0.006 | 173.31M | nan | — | — |
| 10,000 | 0.059 | 168.33M | 0.057 | 175.88M | nan | — | — |
| 100,000 | 0.566 | 176.66M | 0.555 | 180.13M | nan | — | — |
| 1,000,000 | 6.451 | 155.02M | 5.722 | 174.76M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.565 ms**; native kernel **0.549 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.169 | 5.93M | nan | — | — |
| 100,000 | 10 | 0.757 | 0.547 | 18.28M | nan | — | — |
| 100,000 | 1,000 | 6.976 | 6.933 | 144.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 127.42M | 143.42M | 1.00× | 2.12M | 2.81M | 1.00× | — |
| 2 | 147.95M | 158.28M | 1.10× | 3.10M | 3.74M | 1.33× | — |
| 4 | 141.27M | 145.51M | 1.01× | 3.08M | 3.77M | 1.34× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

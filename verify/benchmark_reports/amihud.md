# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.79M | 0.009 | 112.95M | nan | — | — |
| 10,000 | 0.089 | 112.37M | 0.073 | 136.86M | nan | — | — |
| 100,000 | 0.847 | 118.13M | 0.677 | 147.62M | nan | — | — |
| 1,000,000 | 8.677 | 115.25M | 6.889 | 145.17M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.838 ms**; native kernel **0.678 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.266 | 0.200 | 4.99M | nan | — | — |
| 100,000 | 10 | 1.535 | 0.825 | 12.12M | nan | — | — |
| 100,000 | 1,000 | 11.373 | 8.753 | 114.24M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 119.57M | 139.98M | 1.00× | 3.43M | 3.72M | 1.00× | — |
| 2 | 218.83M | 252.48M | 1.80× | 3.04M | 3.28M | 0.88× | — |
| 4 | 128.15M | 140.32M | 1.00× | 2.88M | 2.94M | 0.79× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

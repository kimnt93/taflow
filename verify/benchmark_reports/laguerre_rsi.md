# LaguerreRelativeStrengthIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.41M | 0.009 | 113.95M | nan | — | — |
| 10,000 | 0.080 | 125.05M | 0.077 | 130.70M | nan | — | — |
| 100,000 | 0.768 | 130.16M | 0.755 | 132.50M | nan | — | — |
| 1,000,000 | 7.982 | 125.29M | 7.863 | 127.18M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.778 ms**; native kernel **0.745 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.209 | 0.147 | 6.82M | nan | — | — |
| 100,000 | 10 | 0.774 | 0.457 | 21.89M | nan | — | — |
| 100,000 | 1,000 | 9.980 | 8.456 | 118.25M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 108.27M | 113.24M | 1.00× | 3.28M | 2.90M | 1.00× | — |
| 2 | 109.23M | 117.86M | 1.04× | 3.49M | 3.90M | 1.34× | — |
| 4 | 104.32M | 114.70M | 1.01× | 3.05M | 3.44M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

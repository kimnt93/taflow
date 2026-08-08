# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 325.46M | 0.002 | 535.91M | nan | — | — |
| 10,000 | 0.013 | 764.54M | 0.010 | 962.56M | nan | — | — |
| 100,000 | 0.162 | 616.64M | 0.134 | 743.95M | nan | — | — |
| 1,000,000 | 2.935 | 340.66M | 2.149 | 465.27M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.162 ms**; native kernel **0.138 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.162 | 6.17M | nan | — | — |
| 100,000 | 10 | 1.003 | 0.509 | 19.65M | nan | — | — |
| 100,000 | 1,000 | 3.637 | 2.779 | 359.82M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 259.26M | 405.05M | 1.00× | 2.97M | 3.85M | 1.00× | — |
| 2 | 457.14M | 621.37M | 1.53× | 3.36M | 3.44M | 0.89× | — |
| 4 | 392.14M | 696.43M | 1.72× | 3.02M | 3.28M | 0.85× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

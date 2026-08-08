# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.86M | 0.004 | 254.60M | nan | — | — |
| 10,000 | 0.034 | 292.50M | 0.030 | 329.73M | nan | — | — |
| 100,000 | 0.311 | 321.60M | 0.265 | 377.91M | nan | — | — |
| 1,000,000 | 3.581 | 279.26M | 3.134 | 319.09M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.301 ms**; native kernel **0.285 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.203 | 4.93M | nan | — | — |
| 100,000 | 10 | 1.429 | 0.862 | 11.59M | nan | — | — |
| 100,000 | 1,000 | 5.898 | 5.131 | 194.91M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 229.64M | 293.37M | 1.00× | 2.90M | 3.41M | 1.00× | — |
| 2 | 374.74M | 576.71M | 1.97× | 2.97M | 3.13M | 0.92× | — |
| 4 | 543.92M | 914.77M | 3.12× | 3.22M | 3.39M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

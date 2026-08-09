# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.78M | 0.006 | 168.18M | nan | — | — |
| 10,000 | 0.039 | 258.66M | 0.035 | 284.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.242 | 0.159 | 6.28M | nan | — | — |
| 1,500 | 10 | 0.954 | 0.533 | 18.75M | nan | — | — |
| 1,500 | 100 | 2.049 | 1.426 | 70.11M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.29M | 18.05M | 1.00× | 1.38M | 1.33M | 1.00× | — |
| 2 | 19.70M | 23.45M | 1.30× | 1.62M | 1.74M | 1.31× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

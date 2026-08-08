# ExponentiallyWeightedVariance benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.91M | 0.005 | 189.32M | nan | — | — |
| 10,000 | 0.047 | 211.85M | 0.044 | 228.04M | nan | — | — |
| 100,000 | 0.418 | 238.98M | 0.417 | 240.00M | nan | — | — |
| 1,000,000 | 4.849 | 206.24M | 4.322 | 231.38M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.469 ms**; native kernel **0.414 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.203 | 0.182 | 5.49M | nan | — | — |
| 100,000 | 10 | 0.916 | 0.935 | 10.70M | nan | — | — |
| 100,000 | 1,000 | 8.301 | 6.223 | 160.68M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.30M | 198.40M | 1.00× | 3.89M | 3.60M | 1.00× | — |
| 2 | 287.22M | 379.89M | 1.91× | 3.70M | 3.92M | 1.09× | — |
| 4 | 305.45M | 340.08M | 1.71× | 3.36M | 3.90M | 1.08× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

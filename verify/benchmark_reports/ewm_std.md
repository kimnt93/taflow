# ExponentiallyWeightedStandardDeviation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.09M | 0.005 | 200.99M | nan | — | — |
| 10,000 | 0.042 | 235.96M | 0.039 | 253.73M | nan | — | — |
| 100,000 | 0.414 | 241.53M | 0.384 | 260.72M | nan | — | — |
| 1,000,000 | 4.302 | 232.46M | 3.978 | 251.39M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.417 ms**; native kernel **0.386 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.213 | 0.159 | 6.28M | nan | — | — |
| 100,000 | 10 | 0.830 | 0.506 | 19.76M | nan | — | — |
| 100,000 | 1,000 | 5.774 | 5.037 | 198.53M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 169.64M | 177.63M | 1.00× | 3.63M | 3.47M | 1.00× | — |
| 2 | 325.15M | 386.06M | 2.17× | 3.58M | 3.84M | 1.11× | — |
| 4 | 286.02M | 383.72M | 2.16× | 3.60M | 3.40M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

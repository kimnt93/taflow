# PositiveVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.53M | 0.004 | 244.18M | nan | — | — |
| 10,000 | 0.054 | 184.00M | 0.052 | 191.66M | nan | — | — |
| 100,000 | 0.569 | 175.82M | 0.536 | 186.43M | nan | — | — |
| 1,000,000 | 6.083 | 164.39M | 5.537 | 180.60M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.578 ms**; native kernel **0.591 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.183 | 5.46M | nan | — | — |
| 100,000 | 10 | 1.528 | 0.732 | 13.65M | nan | — | — |
| 100,000 | 1,000 | 8.636 | 6.865 | 145.66M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 126.95M | 132.58M | 1.00× | 2.68M | 2.74M | 1.00× | — |
| 2 | 254.61M | 260.67M | 1.97× | 3.02M | 3.37M | 1.23× | — |
| 4 | 411.03M | 524.58M | 3.96× | 3.16M | 3.23M | 1.18× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

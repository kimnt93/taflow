# RollingEntropy benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.666 | 1.50M | 0.662 | 1.51M | nan | — | — |
| 10,000 | 6.755 | 1.48M | 6.738 | 1.48M | nan | — | — |
| 100,000 | 67.427 | 1.48M | 66.544 | 1.50M | nan | — | — |
| 1,000,000 | 676.058 | 1.48M | 674.066 | 1.48M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **67.500 ms**; native kernel **67.570 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.899 | 0.842 | 1.19M | nan | — | — |
| 100,000 | 10 | 7.712 | 7.330 | 1.36M | nan | — | — |
| 100,000 | 1,000 | 701.034 | 687.202 | 1.46M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.43M | 1.45M | 1.00× | 927.76K | 900.92K | 1.00× | — |
| 2 | 2.81M | 2.76M | 1.89× | 931.94K | 973.43K | 1.08× | — |
| 4 | 5.40M | 5.44M | 3.74× | 927.12K | 955.98K | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

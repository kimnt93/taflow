# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.02M | 0.009 | 106.76M | nan | — | — |
| 10,000 | 0.114 | 88.08M | 0.113 | 88.47M | nan | — | — |
| 100,000 | 1.139 | 87.82M | 1.087 | 92.02M | nan | — | — |
| 1,000,000 | 11.459 | 87.27M | 11.052 | 90.48M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.130 ms**; native kernel **1.082 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.232 | 0.162 | 6.19M | nan | — | — |
| 100,000 | 10 | 0.833 | 0.606 | 16.51M | nan | — | — |
| 100,000 | 1,000 | 12.570 | 12.120 | 82.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.37M | 81.47M | 1.00× | 2.36M | 4.02M | 1.00× | — |
| 2 | 95.27M | 158.33M | 1.94× | 2.75M | 3.85M | 0.96× | — |
| 4 | 227.72M | 294.75M | 3.62× | 2.61M | 2.88M | 0.72× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

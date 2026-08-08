# VolumePriceTrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 197.72M | 0.004 | 279.04M | nan | — | — |
| 10,000 | 0.029 | 340.93M | 0.027 | 374.67M | nan | — | — |
| 100,000 | 0.285 | 350.57M | 0.256 | 390.94M | nan | — | — |
| 1,000,000 | 3.929 | 254.52M | 3.105 | 322.07M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.373 ms**; native kernel **0.285 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.283 | 0.212 | 4.73M | nan | — | — |
| 100,000 | 10 | 1.692 | 0.795 | 12.58M | nan | — | — |
| 100,000 | 1,000 | 5.644 | 6.346 | 157.58M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 146.47M | 190.40M | 1.00× | 2.76M | 2.22M | 1.00× | — |
| 2 | 336.95M | 367.54M | 1.93× | 3.03M | 2.79M | 1.26× | — |
| 4 | 454.14M | 872.01M | 4.58× | 2.88M | 3.18M | 1.43× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

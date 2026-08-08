# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.79M | 0.005 | 211.54M | nan | — | — |
| 10,000 | 0.074 | 134.82M | 0.055 | 182.79M | nan | — | — |
| 100,000 | 0.639 | 156.52M | 0.600 | 166.67M | nan | — | — |
| 1,000,000 | 7.361 | 135.84M | 6.108 | 163.72M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.697 ms**; native kernel **0.585 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.260 | 0.222 | 4.51M | nan | — | — |
| 100,000 | 10 | 1.540 | 0.797 | 12.55M | nan | — | — |
| 100,000 | 1,000 | 8.966 | 10.310 | 97.00M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 119.27M | 146.00M | 1.00× | 2.99M | 2.49M | 1.00× | — |
| 2 | 183.42M | 300.69M | 2.06× | 3.24M | 2.99M | 1.20× | — |
| 4 | 356.91M | 459.80M | 3.15× | 3.20M | 2.96M | 1.19× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# SessionVolumeLevels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.48M | 0.052 | 19.21M | nan | — | — |
| 10,000 | 0.531 | 18.83M | 0.562 | 17.78M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.082 ms**; native kernel **0.080 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.628 | 0.545 | 1.83M | nan | — | — |
| 1,500 | 10 | 2.892 | 2.089 | 4.79M | nan | — | — |
| 1,500 | 100 | 10.625 | 9.643 | 10.37M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.81M | 9.60M | 1.00× | 963.14K | 913.11K | 1.00× | — |
| 2 | 7.47M | 8.88M | 0.93× | 831.88K | 922.51K | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

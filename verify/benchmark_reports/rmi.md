# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.20M | 0.008 | 126.59M | nan | — | — |
| 10,000 | 0.075 | 133.62M | 0.071 | 141.24M | nan | — | — |
| 100,000 | 0.749 | 133.43M | 0.715 | 139.84M | nan | — | — |
| 1,000,000 | 7.775 | 128.62M | 6.834 | 146.33M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.708 ms**; native kernel **0.691 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.173 | 5.79M | nan | — | — |
| 100,000 | 10 | 0.836 | 0.573 | 17.47M | nan | — | — |
| 100,000 | 1,000 | 8.633 | 8.595 | 116.35M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.87M | 128.20M | 1.00× | 2.87M | 3.04M | 1.00× | — |
| 2 | 224.27M | 247.26M | 1.93× | 2.88M | 3.69M | 1.21× | — |
| 4 | 248.40M | 223.83M | 1.75× | 2.91M | 3.21M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

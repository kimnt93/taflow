# RelativeMomentumIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.39M | 0.009 | 109.78M | nan | — | — |
| 10,000 | 0.075 | 133.09M | 0.073 | 137.81M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.284 | 0.189 | 5.28M | nan | — | — |
| 1,500 | 10 | 0.948 | 0.684 | 14.61M | nan | — | — |
| 1,500 | 100 | 2.790 | 2.374 | 42.13M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 12.12M | 16.07M | 1.00× | 893.99K | 1.05M | 1.00× | — |
| 2 | 20.32M | 22.84M | 1.42× | 1.51M | 1.72M | 1.64× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

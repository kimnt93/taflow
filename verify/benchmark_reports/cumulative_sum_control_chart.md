# CumulativeSumControlChart benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.57M | 0.006 | 165.81M | nan | — | — |
| 10,000 | 0.040 | 249.86M | 0.036 | 278.97M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.008 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.226 | 0.158 | 6.32M | nan | — | — |
| 1,500 | 10 | 0.923 | 0.523 | 19.12M | nan | — | — |
| 1,500 | 100 | 1.869 | 1.422 | 70.32M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.99M | 12.79M | 1.00× | 1.27M | 1.43M | 1.00× | — |
| 2 | 15.55M | 22.61M | 1.77× | 1.77M | 1.40M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

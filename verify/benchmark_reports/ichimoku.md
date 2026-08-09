# Ichimoku benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.093 | 10.77M | 0.086 | 11.68M | nan | — | — |
| 10,000 | 0.933 | 10.72M | 0.879 | 11.38M | nan | — | — |
| 100,000 | 9.465 | 10.56M | 8.819 | 11.34M | nan | — | — |
| 1,000,000 | 111.729 | 8.95M | 90.042 | 11.11M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.039 ms**; native kernel **8.904 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.423 | 0.406 | 2.46M | nan | — | — |
| 100,000 | 10 | 2.996 | 1.870 | 5.35M | nan | — | — |
| 100,000 | 1,000 | 95.910 | 94.897 | 10.54M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.37M | 10.53M | 1.00× | 1.69M | 1.78M | 1.00× | — |
| 2 | 19.27M | 20.86M | 1.98× | 1.84M | 1.76M | 0.99× | — |
| 4 | 33.29M | 38.72M | 3.68× | 1.77M | 1.78M | 1.00× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

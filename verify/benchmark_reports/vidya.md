# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.69M | 0.063 | 15.85M | nan | — | — |
| 10,000 | 0.628 | 15.91M | 0.645 | 15.49M | nan | — | — |
| 100,000 | 6.294 | 15.89M | 6.785 | 14.74M | nan | — | — |
| 1,000,000 | 64.216 | 15.57M | 64.061 | 15.61M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.347 ms**; native kernel **6.347 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.218 | 4.59M | nan | — | — |
| 100,000 | 10 | 1.369 | 1.227 | 8.15M | nan | — | — |
| 100,000 | 1,000 | 64.655 | 64.599 | 15.48M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.13M | 15.45M | 1.00× | 2.15M | 2.56M | 1.00× | — |
| 2 | 28.43M | 28.95M | 1.87× | 2.39M | 2.63M | 1.03× | — |
| 4 | 52.43M | 51.39M | 3.33× | 2.13M | 2.51M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

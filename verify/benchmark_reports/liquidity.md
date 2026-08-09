# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.043 | 23.08M | 0.039 | 25.60M | nan | — | — |
| 10,000 | 0.435 | 22.97M | 0.438 | 22.84M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.060 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.376 | 0.296 | 3.38M | nan | — | — |
| 1,500 | 10 | 1.998 | 1.206 | 8.29M | nan | — | — |
| 1,500 | 100 | 6.477 | 5.511 | 18.14M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.50M | 8.63M | 1.00× | 1.14M | 923.84K | 1.00× | — |
| 2 | 12.21M | 14.29M | 1.66× | 1.26M | 1.26M | 1.36× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

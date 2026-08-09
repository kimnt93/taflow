# GapUp benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 216.09M | 0.003 | 297.17M | nan | — | — |
| 10,000 | 0.029 | 347.04M | 0.026 | 390.65M | nan | — | — |
| 100,000 | 0.269 | 372.04M | 0.239 | 418.39M | nan | — | — |
| 1,000,000 | 3.058 | 327.06M | 2.785 | 359.01M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.265 ms**; native kernel **0.240 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.171 | 5.85M | nan | — | — |
| 100,000 | 10 | 1.377 | 0.685 | 14.60M | nan | — | — |
| 100,000 | 1,000 | 5.002 | 3.940 | 253.80M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 178.77M | 285.71M | 1.00× | 2.74M | 3.35M | 1.00× | — |
| 2 | 438.41M | 456.98M | 1.60× | 3.12M | 3.53M | 1.05× | — |
| 4 | 555.87M | 980.10M | 3.43× | 3.35M | 3.46M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

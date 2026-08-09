# ValueWhen benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 267.16M | 0.003 | 390.39M | nan | — | — |
| 10,000 | 0.020 | 491.00M | 0.018 | 556.44M | nan | — | — |
| 100,000 | 0.188 | 533.16M | 0.163 | 614.97M | nan | — | — |
| 1,000,000 | 2.145 | 466.23M | 1.746 | 572.77M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.188 ms**; native kernel **0.164 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.322 | 0.264 | 3.79M | nan | — | — |
| 100,000 | 10 | 1.191 | 0.693 | 14.44M | nan | — | — |
| 100,000 | 1,000 | 3.931 | 3.053 | 327.56M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 239.63M | 293.94M | 1.00× | 2.54M | 2.38M | 1.00× | — |
| 2 | 472.02M | 690.57M | 2.35× | 2.62M | 2.55M | 1.07× | — |
| 4 | 595.79M | 1.14G | 3.87× | 2.35M | 2.59M | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

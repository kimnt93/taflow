# CloseToCloseSigma benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.10M | 0.019 | 53.16M | nan | — | — |
| 10,000 | 0.183 | 54.56M | 0.183 | 54.69M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.029 ms**; native kernel **0.028 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.292 | 0.202 | 4.94M | nan | — | — |
| 1,500 | 10 | 1.261 | 0.708 | 14.13M | nan | — | — |
| 1,500 | 100 | 4.068 | 3.387 | 29.52M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

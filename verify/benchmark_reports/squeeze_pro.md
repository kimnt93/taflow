# SqueezePro benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.85M | 0.045 | 22.43M | nan | — | — |
| 10,000 | 0.429 | 23.32M | 0.413 | 24.18M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.066 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.695 | 0.386 | 2.59M | nan | — | — |
| 1,500 | 10 | 2.577 | 1.402 | 7.13M | nan | — | — |
| 1,500 | 100 | 8.161 | 6.281 | 15.92M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

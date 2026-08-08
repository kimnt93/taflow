# Vortex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.22M | 0.023 | 42.66M | nan | — | — |
| 10,000 | 0.218 | 45.97M | 0.212 | 47.25M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.035 ms**; native kernel **0.034 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.436 | 0.317 | 3.16M | nan | — | — |
| 1,500 | 10 | 2.409 | 1.211 | 8.26M | nan | — | — |
| 1,500 | 100 | 6.394 | 4.531 | 22.07M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

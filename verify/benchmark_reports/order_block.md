# OrderBlock benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.234 | 4.27M | 0.222 | 4.50M | nan | — | — |
| 10,000 | 2.509 | 3.99M | 2.548 | 3.93M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.358 ms**; native kernel **0.359 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.666 | 0.555 | 1.80M | nan | — | — |
| 1,500 | 10 | 4.809 | 3.415 | 2.93M | nan | — | — |
| 1,500 | 100 | 27.109 | 25.219 | 3.97M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

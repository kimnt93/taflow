# Crossover benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.82M | 0.004 | 249.64M | nan | — | — |
| 10,000 | 0.040 | 252.71M | 0.032 | 309.92M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.307 | 0.216 | 4.64M | nan | — | — |
| 1,500 | 10 | 1.587 | 0.752 | 13.30M | nan | — | — |
| 1,500 | 100 | 3.251 | 2.154 | 46.43M | nan | — | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

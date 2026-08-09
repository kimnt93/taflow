# Vortex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.86M | 0.016 | 63.18M | nan | — | — |
| 10,000 | 0.117 | 85.42M | 0.117 | 85.52M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.024 ms**; native kernel **0.022 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.377 | 0.278 | 3.59M | nan | — | — |
| 1,500 | 10 | 2.073 | 1.289 | 7.76M | nan | — | — |
| 1,500 | 100 | 3.945 | 2.824 | 35.41M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.45M | 9.94M | 1.00× | 818.09K | 1.20M | 1.00× | — |
| 2 | 17.10M | 19.74M | 1.99× | 1.23M | 1.13M | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

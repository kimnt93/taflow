# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.31M | 0.006 | 165.44M | nan | — | — |
| 10,000 | 0.054 | 185.31M | 0.052 | 192.24M | nan | — | — |
| 100,000 | 0.526 | 190.12M | 0.476 | 210.27M | nan | — | — |
| 1,000,000 | 5.594 | 178.75M | 5.062 | 197.55M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.501 ms**; native kernel **0.458 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.232 | 0.162 | 6.16M | nan | — | — |
| 100,000 | 10 | 1.025 | 0.581 | 17.22M | nan | — | — |
| 100,000 | 1,000 | 6.906 | 5.916 | 169.04M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 163.59M | 176.61M | 1.00× | 2.91M | 3.04M | 1.00× | — |
| 2 | 308.16M | 384.73M | 2.18× | 3.72M | 3.82M | 1.26× | — |
| 4 | 427.86M | 662.78M | 3.75× | 3.52M | 3.68M | 1.21× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

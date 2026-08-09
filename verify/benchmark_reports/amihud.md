# Amihud benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.73M | 0.010 | 100.25M | nan | — | — |
| 10,000 | 0.068 | 147.02M | 0.065 | 153.45M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.013 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.274 | 0.200 | 4.99M | nan | — | — |
| 1,500 | 10 | 1.496 | 0.799 | 12.51M | nan | — | — |
| 1,500 | 100 | 2.857 | 2.000 | 49.99M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.39M | 15.97M | 1.00× | 1.39M | 862.69K | 1.00× | — |
| 2 | 12.76M | 15.98M | 1.00× | 1.26M | 939.89K | 1.09× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

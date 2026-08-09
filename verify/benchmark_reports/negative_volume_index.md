# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.25M | 0.006 | 171.27M | nan | — | — |
| 10,000 | 0.062 | 162.07M | 0.057 | 174.80M | nan | — | — |
| 100,000 | 0.578 | 172.89M | 0.550 | 181.73M | nan | — | — |
| 1,000,000 | 6.161 | 162.30M | 5.786 | 172.83M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.586 ms**; native kernel **0.551 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.241 | 0.189 | 5.29M | nan | — | — |
| 100,000 | 10 | 1.512 | 0.817 | 12.24M | nan | — | — |
| 100,000 | 1,000 | 8.035 | 7.017 | 142.51M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.98M | 158.99M | 1.00× | 3.00M | 3.48M | 1.00× | — |
| 2 | 240.75M | 262.18M | 1.65× | 3.11M | 3.03M | 0.87× | — |
| 4 | 375.45M | 223.32M | 1.40× | 2.94M | 3.18M | 0.91× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

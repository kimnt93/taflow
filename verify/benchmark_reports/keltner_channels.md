# KeltnerChannels benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 60.20M | 0.014 | 73.42M | nan | — | — |
| 10,000 | 0.100 | 99.73M | 0.090 | 110.87M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.021 ms**; native kernel **0.018 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.429 | 0.304 | 3.29M | nan | — | — |
| 1,500 | 10 | 2.074 | 1.052 | 9.51M | nan | — | — |
| 1,500 | 100 | 3.687 | 2.528 | 39.55M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.84M | 9.97M | 1.00× | 1.19M | 1.19M | 1.00× | — |
| 2 | 16.69M | 16.74M | 1.68× | 1.02M | 1.16M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

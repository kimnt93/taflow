# Rising benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 120.10M | 0.007 | 136.42M | nan | — | — |
| 10,000 | 0.053 | 189.65M | 0.049 | 203.50M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.239 | 0.171 | 5.85M | nan | — | — |
| 1,500 | 10 | 1.005 | 0.585 | 17.08M | nan | — | — |
| 1,500 | 100 | 2.206 | 1.654 | 60.47M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.66M | 17.25M | 1.00× | 1.54M | 1.73M | 1.00× | — |
| 2 | 15.19M | 20.29M | 1.18× | 1.08M | 1.69M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

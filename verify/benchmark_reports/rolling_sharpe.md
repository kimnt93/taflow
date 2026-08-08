# RollingSharpe benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.52M | 0.020 | 51.28M | nan | — | — |
| 10,000 | 0.205 | 48.88M | 0.201 | 49.80M | nan | — | — |
| 100,000 | 2.060 | 48.54M | 1.959 | 51.04M | nan | — | — |
| 1,000,000 | 20.368 | 49.10M | 19.154 | 52.21M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.992 ms**; native kernel **2.165 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.244 | 0.175 | 5.72M | nan | — | — |
| 100,000 | 10 | 1.105 | 0.667 | 15.00M | nan | — | — |
| 100,000 | 1,000 | 20.456 | 18.992 | 52.65M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 42.15M | 48.09M | 1.00× | 2.61M | 3.05M | 1.00× | — |
| 2 | 70.57M | 60.49M | 1.26× | 2.37M | 3.08M | 1.01× | — |
| 4 | 95.22M | 104.84M | 2.18× | 2.93M | 2.99M | 0.98× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

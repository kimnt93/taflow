# Ichimoku benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.104 | 9.64M | 0.096 | 10.41M | nan | — | — |
| 10,000 | 0.980 | 10.21M | 0.942 | 10.62M | nan | — | — |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.148 ms**; native kernel **0.147 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.545 | 0.403 | 2.48M | nan | — | — |
| 1,500 | 10 | 3.702 | 2.013 | 4.97M | nan | — | — |
| 1,500 | 100 | 12.655 | 11.174 | 8.95M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 4.90M | 5.88M | 1.00× | 851.10K | 910.31K | 1.00× | — |
| 2 | 8.81M | 8.84M | 1.50× | 1.13M | 853.21K | 0.94× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

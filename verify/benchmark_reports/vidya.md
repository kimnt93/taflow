# VariableIndexDynamicAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.58M | 0.062 | 16.24M | nan | — | — |
| 10,000 | 0.666 | 15.01M | 0.643 | 15.56M | nan | — | — |
| 100,000 | 6.479 | 15.43M | 6.660 | 15.02M | nan | — | — |
| 1,000,000 | 66.424 | 15.05M | 65.312 | 15.31M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.530 ms**; native kernel **6.280 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.304 | 0.234 | 4.28M | nan | — | — |
| 100,000 | 10 | 1.292 | 1.181 | 8.47M | nan | — | — |
| 100,000 | 1,000 | 64.337 | 60.423 | 16.55M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.51M | 14.09M | 1.00× | 2.33M | 2.59M | 1.00× | — |
| 2 | 14.39M | 14.51M | 1.03× | 2.04M | 2.61M | 1.01× | — |
| 4 | 14.65M | 14.71M | 1.04× | 2.05M | 2.57M | 0.99× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

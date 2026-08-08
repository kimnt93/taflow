# FractalDimension benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.38M | 0.057 | 17.44M | nan | — | — |
| 10,000 | 0.601 | 16.64M | 0.578 | 17.30M | nan | — | — |
| 100,000 | 5.780 | 17.30M | 5.570 | 17.95M | nan | — | — |
| 1,000,000 | 57.446 | 17.41M | 56.562 | 17.68M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.771 ms**; native kernel **5.585 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.261 | 0.208 | 4.82M | nan | — | — |
| 100,000 | 10 | 1.423 | 1.138 | 8.78M | nan | — | — |
| 100,000 | 1,000 | 67.591 | 63.655 | 15.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 16.65M | 17.09M | 1.00× | 2.69M | 2.80M | 1.00× | — |
| 2 | 30.59M | 32.02M | 1.87× | 2.54M | 2.66M | 0.95× | — |
| 4 | 53.68M | 55.81M | 3.27× | 2.48M | 2.66M | 0.95× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RollingMedian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.34M | 0.028 | 35.23M | nan | — | — |
| 10,000 | 0.336 | 29.75M | 0.324 | 30.88M | nan | — | — |
| 100,000 | 3.205 | 31.20M | 3.191 | 31.34M | nan | — | — |
| 1,000,000 | 33.338 | 30.00M | 32.256 | 31.00M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.220 ms**; native kernel **3.207 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.196 | 5.10M | nan | — | — |
| 100,000 | 10 | 1.214 | 0.837 | 11.95M | nan | — | — |
| 100,000 | 1,000 | 33.223 | 33.432 | 29.91M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 28.85M | 30.38M | 1.00× | 2.47M | 3.44M | 1.00× | — |
| 2 | 54.16M | 57.62M | 1.90× | 2.66M | 3.12M | 0.91× | — |
| 4 | 103.66M | 103.49M | 3.41× | 2.77M | 2.98M | 0.87× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

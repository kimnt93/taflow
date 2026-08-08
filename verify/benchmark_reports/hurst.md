# Hurst benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.24M | 0.056 | 17.81M | nan | — | — |
| 10,000 | 0.580 | 17.23M | 0.559 | 17.91M | nan | — | — |
| 100,000 | 5.772 | 17.33M | 5.689 | 17.58M | nan | — | — |
| 1,000,000 | 58.937 | 16.97M | 60.754 | 16.46M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **5.849 ms**; native kernel **5.746 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.264 | 0.205 | 4.88M | nan | — | — |
| 100,000 | 10 | 1.613 | 1.062 | 9.42M | nan | — | — |
| 100,000 | 1,000 | 62.212 | 72.237 | 13.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 16.54M | 16.88M | 1.00× | 2.00M | 2.64M | 1.00× | — |
| 2 | 32.72M | 32.93M | 1.95× | 2.57M | 2.84M | 1.08× | — |
| 4 | 60.16M | 63.63M | 3.77× | 2.66M | 2.77M | 1.05× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

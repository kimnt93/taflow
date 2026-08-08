# SqueezePro benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.77M | 0.034 | 29.70M | nan | — | — |
| 10,000 | 0.303 | 33.01M | 0.299 | 33.48M | nan | — | — |
| 100,000 | 3.295 | 30.35M | 3.077 | 32.50M | nan | — | — |
| 1,000,000 | 53.492 | 18.69M | 48.412 | 20.66M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.098 ms**; native kernel **2.915 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.310 | 3.23M | nan | — | — |
| 100,000 | 10 | 2.243 | 1.195 | 8.37M | nan | — | — |
| 100,000 | 1,000 | 33.133 | 30.500 | 32.79M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 27.12M | 32.82M | 1.00× | 1.93M | 1.56M | 1.00× | — |
| 2 | 46.71M | 60.40M | 1.84× | 2.06M | 1.96M | 1.26× | — |
| 4 | 68.33M | 82.62M | 2.52× | 1.92M | 1.84M | 1.18× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

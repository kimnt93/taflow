# SqueezePro benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.40M | 0.042 | 23.55M | nan | — | — |
| 10,000 | 0.417 | 23.96M | 0.398 | 25.14M | nan | — | — |
| 100,000 | 4.325 | 23.12M | 4.050 | 24.69M | nan | — | — |
| 1,000,000 | 66.515 | 15.03M | 56.888 | 17.58M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.278 ms**; native kernel **3.924 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.356 | 0.303 | 3.30M | nan | — | — |
| 100,000 | 10 | 2.446 | 1.332 | 7.51M | nan | — | — |
| 100,000 | 1,000 | 43.786 | 39.788 | 25.13M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 20.81M | 22.95M | 1.00× | 1.97M | 1.80M | 1.00× | — |
| 2 | 36.52M | 38.61M | 1.68× | 2.00M | 1.98M | 1.10× | — |
| 4 | 54.62M | 73.78M | 3.21× | 2.02M | 1.91M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

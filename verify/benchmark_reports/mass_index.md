# MassIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.34M | 0.007 | 145.37M | nan | — | — |
| 10,000 | 0.060 | 165.73M | 0.064 | 155.77M | nan | — | — |
| 100,000 | 0.584 | 171.35M | 0.544 | 183.91M | nan | — | — |
| 1,000,000 | 6.178 | 161.87M | 5.770 | 173.30M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.569 ms**; native kernel **0.537 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.254 | 0.189 | 5.29M | nan | — | — |
| 100,000 | 10 | 1.400 | 0.708 | 14.13M | nan | — | — |
| 100,000 | 1,000 | 9.132 | 6.853 | 145.92M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.38M | 145.54M | 1.00× | 3.12M | 2.93M | 1.00× | — |
| 2 | 231.96M | 293.21M | 2.01× | 3.18M | 3.40M | 1.16× | — |
| 4 | 151.93M | 162.66M | 1.12× | 3.15M | 3.12M | 1.06× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

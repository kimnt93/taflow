# OrnsteinUhlenbeckHalfLife benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.62M | 0.044 | 22.89M | nan | — | — |
| 10,000 | 0.438 | 22.82M | 0.467 | 21.43M | nan | — | — |
| 100,000 | 4.356 | 22.96M | 4.385 | 22.80M | nan | — | — |
| 1,000,000 | 44.164 | 22.64M | 43.934 | 22.76M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.551 ms**; native kernel **4.338 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.226 | 0.185 | 5.42M | nan | — | — |
| 100,000 | 10 | 1.242 | 0.951 | 10.51M | nan | — | — |
| 100,000 | 1,000 | 49.588 | 49.849 | 20.06M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 20.97M | 21.22M | 1.00× | 2.84M | 2.91M | 1.00× | — |
| 2 | 32.93M | 32.90M | 1.55× | 2.86M | 2.96M | 1.02× | — |
| 4 | 61.51M | 60.84M | 2.87× | 2.64M | 3.00M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

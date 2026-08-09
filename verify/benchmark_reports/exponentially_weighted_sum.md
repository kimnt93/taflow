# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.43M | 0.004 | 239.98M | nan | — | — |
| 10,000 | 0.036 | 274.50M | 0.033 | 301.75M | nan | — | — |
| 100,000 | 0.334 | 299.79M | 0.312 | 320.54M | nan | — | — |
| 1,000,000 | 3.537 | 282.73M | 3.151 | 317.37M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.338 ms**; native kernel **0.330 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.475 | 0.255 | 3.92M | nan | — | — |
| 100,000 | 10 | 0.890 | 0.489 | 20.45M | nan | — | — |
| 100,000 | 1,000 | 5.155 | 4.351 | 229.85M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 201.67M | 236.65M | 1.00× | 3.14M | 3.90M | 1.00× | — |
| 2 | 374.56M | 405.84M | 1.71× | 3.42M | 3.66M | 0.94× | — |
| 4 | 404.57M | 737.87M | 3.12× | 3.55M | 4.00M | 1.02× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

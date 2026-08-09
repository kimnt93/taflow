# GarmanKlass benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.53M | 0.017 | 60.09M | nan | — | — |
| 10,000 | 0.152 | 65.84M | 0.149 | 67.21M | nan | — | — |
| 100,000 | 1.513 | 66.08M | 1.475 | 67.80M | nan | — | — |
| 1,000,000 | 15.422 | 64.84M | 14.809 | 67.53M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.538 ms**; native kernel **1.513 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.251 | 3.98M | nan | — | — |
| 100,000 | 10 | 2.373 | 1.164 | 8.59M | nan | — | — |
| 100,000 | 1,000 | 18.642 | 16.196 | 61.74M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.26M | 57.76M | 1.00× | 2.31M | 2.32M | 1.00× | — |
| 2 | 103.85M | 105.95M | 1.83× | 2.27M | 2.54M | 1.09× | — |
| 4 | 163.85M | 217.92M | 3.77× | 2.22M | 2.38M | 1.03× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# OrderBlock benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.084 | 11.95M | 0.076 | 13.18M | nan | — | — |
| 10,000 | 0.837 | 11.95M | 0.816 | 12.26M | nan | — | — |
| 100,000 | 9.910 | 10.09M | 8.647 | 11.56M | nan | — | — |
| 1,000,000 | 108.488 | 9.22M | 98.209 | 10.18M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.621 ms**; native kernel **9.275 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.514 | 0.430 | 2.33M | nan | — | — |
| 100,000 | 10 | 4.051 | 2.238 | 4.47M | nan | — | — |
| 100,000 | 1,000 | 100.140 | 92.248 | 10.84M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.91M | 11.49M | 1.00× | 1.64M | 1.50M | 1.00× | — |
| 2 | 18.98M | 21.00M | 1.83× | 1.62M | 1.59M | 1.07× | — |
| 4 | 34.55M | 38.11M | 3.32× | 1.60M | 1.55M | 1.04× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

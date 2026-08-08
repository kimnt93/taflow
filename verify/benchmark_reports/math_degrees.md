# MathDegrees benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 289.34M | 0.002 | 516.15M | nan | — | — |
| 10,000 | 0.014 | 720.84M | 0.011 | 907.31M | nan | — | — |
| 100,000 | 0.160 | 626.09M | 0.133 | 749.40M | nan | — | — |
| 1,000,000 | 2.797 | 357.54M | 2.258 | 442.87M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.159 ms**; native kernel **0.132 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.218 | 0.143 | 6.99M | nan | — | — |
| 100,000 | 10 | 0.848 | 0.500 | 20.02M | nan | — | — |
| 100,000 | 1,000 | 3.530 | 2.694 | 371.14M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 273.91M | 334.20M | 1.00× | 3.45M | 2.98M | 1.00× | — |
| 2 | 446.56M | 626.13M | 1.87× | 3.18M | 3.45M | 1.16× | — |
| 4 | 405.90M | 736.24M | 2.20× | 2.95M | 3.01M | 1.01× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

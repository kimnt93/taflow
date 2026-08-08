# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.97M | 0.047 | 21.23M | 0.076 | 1.60× | 1.62× |
| 10,000 | 0.476 | 21.01M | 0.462 | 21.62M | 0.472 | 0.99× | 1.02× |
| 100,000 | 4.701 | 21.27M | 4.500 | 22.22M | 4.448 | 0.95× | 0.99× |
| 1,000,000 | 49.466 | 20.22M | 45.619 | 21.92M | 44.428 | 0.90× | 0.97× |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.649 ms**; native kernel **4.493 ms**; TA-Lib 4.410 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.304 | 0.219 | 4.57M | 4418.277 | 20204.87× | 143.94× |
| 100,000 | 10 | 1.358 | 1.006 | 9.94M | 4561.052 | 4533.74× | 31.05× |
| 100,000 | 1,000 | 45.434 | 44.207 | 22.62M | 4372.928 | 98.92× | 1.74× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 19.49M | 19.97M | 1.00× | 2.15M | 2.37M | 1.00× | 20.23M |
| 2 | 36.29M | 37.82M | 1.89× | 2.17M | 2.67M | 1.13× | 20.45M |
| 4 | 66.32M | 78.49M | 3.93× | 2.37M | 2.58M | 1.09× | 21.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.25M | 0.010 | 96.11M | 0.055 | 4.76× | 5.24× |
| 10,000 | 0.112 | 89.44M | 0.113 | 88.76M | 0.094 | 0.84× | 0.84× |
| 100,000 | 1.077 | 92.87M | 1.056 | 94.71M | 0.682 | 0.63× | 0.65× |
| 1,000,000 | 11.682 | 85.60M | 11.349 | 88.11M | 6.293 | 0.54× | 0.55× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.066 ms**; native kernel **1.062 ms**; TA-Lib 0.692 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.290 | 0.216 | 4.63M | 640.348 | 2963.74× | 242.20× |
| 100,000 | 10 | 1.047 | 0.825 | 12.12M | 679.375 | 823.22× | 62.87× |
| 100,000 | 1,000 | 12.957 | 17.038 | 58.69M | 654.826 | 38.43× | 3.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 84.35M | 83.95M | 1.00× | 2.59M | 3.27M | 1.00× | 115.38M |
| 2 | 149.62M | 169.54M | 2.02× | 2.53M | 3.05M | 0.93× | 123.04M |
| 4 | 267.26M | 313.43M | 3.73× | 2.24M | 2.59M | 0.79× | 126.03M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

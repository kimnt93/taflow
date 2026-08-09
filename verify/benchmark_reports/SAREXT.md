# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.22M | 0.012 | 83.83M | 0.053 | 4.08× | 4.43× |
| 10,000 | 0.116 | 86.54M | 0.112 | 89.26M | 0.099 | 0.86× | 0.88× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.018 ms**; native kernel **0.017 ms**; TA-Lib 0.054 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.306 | 0.209 | 4.79M | 55.579 | 266.17× | 234.30× |
| 1,500 | 10 | 1.014 | 0.894 | 11.19M | 55.829 | 62.47× | 60.53× |
| 1,500 | 100 | 3.766 | 3.112 | 32.14M | 57.724 | 18.55× | 20.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.84M | 9.01M | 1.00× | 1.17M | 1.41M | 1.00× | 7.77M |
| 2 | 17.09M | 17.13M | 1.90× | 1.07M | 1.25M | 0.89× | 6.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

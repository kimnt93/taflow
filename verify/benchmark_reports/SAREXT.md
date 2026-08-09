# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.21M | 0.010 | 96.05M | 0.049 | 4.55× | 4.74× |
| 10,000 | 0.111 | 90.13M | 0.105 | 94.94M | 0.090 | 0.81× | 0.85× |
| 100,000 | 1.105 | 90.51M | 1.071 | 93.35M | 0.624 | 0.56× | 0.58× |
| 1,000,000 | 11.350 | 88.11M | 10.696 | 93.49M | 5.943 | 0.52× | 0.56× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.101 ms**; native kernel **1.076 ms**; TA-Lib 0.626 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.267 | 0.188 | 5.32M | 649.425 | 3454.98× | 243.18× |
| 100,000 | 10 | 0.838 | 0.838 | 11.94M | 624.682 | 745.72× | 55.53× |
| 100,000 | 1,000 | 12.746 | 12.534 | 79.78M | 628.452 | 50.14× | 3.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 75.38M | 78.15M | 1.00× | 2.72M | 2.84M | 1.00× | 116.68M |
| 2 | 142.98M | 158.65M | 2.03× | 2.54M | 3.66M | 1.29× | 129.22M |
| 4 | 259.06M | 289.95M | 3.71× | 2.67M | 3.03M | 1.07× | 127.50M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

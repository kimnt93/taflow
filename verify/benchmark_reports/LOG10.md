# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.02M | 0.009 | 108.69M | 0.034 | 3.23× | 3.65× |
| 10,000 | 0.091 | 109.37M | 0.091 | 109.91M | 0.103 | 1.13× | 1.13× |
| 100,000 | 0.885 | 112.96M | 0.870 | 114.91M | 0.807 | 0.91× | 0.93× |
| 1,000,000 | 9.822 | 101.82M | 9.360 | 106.84M | 7.752 | 0.79× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.873 ms**; native kernel **0.866 ms**; TA-Lib 0.780 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.232 | 0.162 | 6.16M | 770.491 | 4749.50× | 148.36× |
| 100,000 | 10 | 0.952 | 0.583 | 17.14M | 763.644 | 1309.18× | 42.20× |
| 100,000 | 1,000 | 11.773 | 10.175 | 98.28M | 766.852 | 75.37× | 3.29× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 87.13M | 100.46M | 1.00× | 2.49M | 2.94M | 1.00× | 102.90M |
| 2 | 179.57M | 180.94M | 1.80× | 2.85M | 3.10M | 1.05× | 105.08M |
| 4 | 253.05M | 332.61M | 3.31× | 2.70M | 3.02M | 1.03× | 108.31M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

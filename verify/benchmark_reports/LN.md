# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.10M | 0.006 | 177.06M | 0.029 | 4.40× | 5.16× |
| 10,000 | 0.053 | 189.52M | 0.050 | 200.14M | 0.068 | 1.29× | 1.36× |
| 100,000 | 0.503 | 198.88M | 0.477 | 209.66M | 0.415 | 0.83× | 0.87× |
| 1,000,000 | 6.010 | 166.40M | 5.482 | 182.43M | 3.961 | 0.66× | 0.72× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.504 ms**; native kernel **0.482 ms**; TA-Lib 0.414 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.243 | 0.152 | 6.56M | 416.306 | 2730.96× | 163.36× |
| 100,000 | 10 | 0.996 | 0.572 | 17.49M | 409.359 | 715.86× | 42.26× |
| 100,000 | 1,000 | 7.467 | 6.245 | 160.12M | 410.937 | 65.80× | 4.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.54M | 165.56M | 1.00× | 2.90M | 3.24M | 1.00× | 157.99M |
| 2 | 268.28M | 321.64M | 1.94× | 3.16M | 3.47M | 1.07× | 190.03M |
| 4 | 319.37M | 419.42M | 2.53× | 2.61M | 3.12M | 0.96× | 187.48M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

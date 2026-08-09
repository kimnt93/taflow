# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.06M | 0.009 | 111.87M | 0.034 | 3.40× | 3.76× |
| 10,000 | 0.085 | 117.21M | 0.083 | 120.90M | 0.102 | 1.20× | 1.24× |
| 100,000 | 0.838 | 119.26M | 0.813 | 122.99M | 0.786 | 0.94× | 0.97× |
| 1,000,000 | 9.035 | 110.67M | 8.769 | 114.04M | 7.635 | 0.84× | 0.87× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.833 ms**; native kernel **0.809 ms**; TA-Lib 0.788 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.167 | 5.98M | 787.693 | 4711.44× | 147.15× |
| 100,000 | 10 | 0.933 | 0.616 | 16.24M | 792.178 | 1286.60× | 39.32× |
| 100,000 | 1,000 | 14.790 | 9.875 | 101.27M | 803.247 | 81.34× | 3.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 96.43M | 104.07M | 1.00× | 3.06M | 2.49M | 1.00× | 104.16M |
| 2 | 189.09M | 212.43M | 2.04× | 2.78M | 3.63M | 1.45× | 107.47M |
| 4 | 272.80M | 348.49M | 3.35× | 2.77M | 3.04M | 1.22× | 107.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

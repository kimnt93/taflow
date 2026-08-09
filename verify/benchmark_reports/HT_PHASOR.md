# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.36M | 0.044 | 22.49M | 0.076 | 1.63× | 1.72× |
| 10,000 | 0.462 | 21.63M | 0.441 | 22.69M | 0.450 | 0.97× | 1.02× |
| 100,000 | 4.497 | 22.24M | 4.353 | 22.97M | 4.183 | 0.93× | 0.96× |
| 1,000,000 | 45.251 | 22.10M | 43.902 | 22.78M | 42.107 | 0.93× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **4.398 ms**; native kernel **4.331 ms**; TA-Lib 4.239 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.302 | 0.213 | 4.69M | 4188.375 | 19628.53× | 143.12× |
| 100,000 | 10 | 1.504 | 0.951 | 10.51M | 4205.162 | 4420.98× | 32.39× |
| 100,000 | 1,000 | 45.705 | 52.275 | 19.13M | 4213.493 | 80.60× | 1.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 21.17M | 20.65M | 1.00× | 2.70M | 2.69M | 1.00× | 20.73M |
| 2 | 38.33M | 40.69M | 1.97× | 2.54M | 2.49M | 0.93× | 21.42M |
| 4 | 68.04M | 78.62M | 3.81× | 2.46M | 2.67M | 0.99× | 21.91M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

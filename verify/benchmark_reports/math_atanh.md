# MathAtanh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.75M | 0.006 | 181.02M | nan | — | — |
| 10,000 | 0.050 | 201.97M | 0.046 | 218.42M | nan | — | — |
| 100,000 | 0.477 | 209.67M | 0.452 | 221.38M | nan | — | — |
| 1,000,000 | 5.712 | 175.06M | 5.246 | 190.63M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.478 ms**; native kernel **0.462 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.252 | 0.173 | 5.78M | nan | — | — |
| 100,000 | 10 | 1.035 | 0.560 | 17.85M | nan | — | — |
| 100,000 | 1,000 | 7.072 | 6.629 | 150.85M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 142.31M | 174.60M | 1.00× | 2.72M | 3.11M | 1.00× | — |
| 2 | 280.30M | 325.74M | 1.87× | 2.96M | 3.76M | 1.21× | — |
| 4 | 341.30M | 464.90M | 2.66× | 2.61M | 3.01M | 0.97× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

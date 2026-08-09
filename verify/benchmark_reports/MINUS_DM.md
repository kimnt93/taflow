# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.40M | 0.008 | 130.35M | 0.036 | 3.99× | 4.76× |
| 10,000 | 0.055 | 180.41M | 0.054 | 185.58M | 0.084 | 1.52× | 1.56× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.258 | 0.193 | 5.17M | 38.551 | 199.38× | 159.10× |
| 1,500 | 10 | 1.560 | 0.796 | 12.56M | 37.772 | 47.45× | 38.57× |
| 1,500 | 100 | 3.655 | 2.332 | 42.88M | 38.852 | 16.66× | 13.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.94M | 17.97M | 1.00× | 1.40M | 1.32M | 1.00× | 9.06M |
| 2 | 19.19M | 15.11M | 0.84× | 1.18M | 1.59M | 1.21× | 9.18M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

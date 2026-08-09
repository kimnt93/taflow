# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.75M | 0.007 | 136.99M | 0.032 | 3.48× | 4.34× |
| 10,000 | 0.061 | 162.74M | 0.059 | 168.84M | 0.083 | 1.35× | 1.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.346 | 0.270 | 3.70M | 34.744 | 128.57× | 106.46× |
| 1,500 | 10 | 2.670 | 1.352 | 7.40M | 34.945 | 25.84× | 21.13× |
| 1,500 | 100 | 5.623 | 3.292 | 30.37M | 35.985 | 10.93× | 8.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.06M | 13.47M | 1.00× | 1.24M | 1.13M | 1.00× | 8.49M |
| 2 | 14.96M | 21.68M | 1.61× | 1.34M | 1.24M | 1.10× | 9.43M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

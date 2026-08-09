# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 156.67M | 0.005 | 184.28M | 0.030 | 4.71× | 5.54× |
| 10,000 | 0.049 | 204.16M | 0.047 | 213.68M | 0.067 | 1.38× | 1.44× |
| 100,000 | 0.476 | 210.17M | 0.452 | 221.42M | 0.422 | 0.89× | 0.93× |
| 1,000,000 | 5.427 | 184.25M | 5.204 | 192.15M | 4.049 | 0.75× | 0.78× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.473 ms**; native kernel **0.448 ms**; TA-Lib 0.426 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.151 | 6.62M | 433.659 | 2870.42× | 165.60× |
| 100,000 | 10 | 0.903 | 0.602 | 16.61M | 418.794 | 695.78× | 42.89× |
| 100,000 | 1,000 | 7.507 | 6.319 | 158.26M | 425.127 | 67.28× | 4.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 150.63M | 188.62M | 1.00× | 3.67M | 3.66M | 1.00× | 180.54M |
| 2 | 292.80M | 317.28M | 1.68× | 3.54M | 3.48M | 0.95× | 196.14M |
| 4 | 350.36M | 537.84M | 2.85× | 2.95M | 3.37M | 0.92× | 194.05M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

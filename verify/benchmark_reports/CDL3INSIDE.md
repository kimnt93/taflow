# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.44M | 0.005 | 204.77M | 0.039 | 5.84× | 7.95× |
| 10,000 | 0.088 | 114.05M | 0.083 | 119.82M | 0.126 | 1.44× | 1.51× |
| 100,000 | 0.994 | 100.61M | 0.993 | 100.71M | 1.022 | 1.03× | 1.03× |
| 1,000,000 | 10.218 | 97.87M | 10.057 | 99.44M | 10.039 | 0.98× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.982 ms**; native kernel **0.972 ms**; TA-Lib 1.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.366 | 0.302 | 3.31M | 1033.937 | 3426.44× | 88.61× |
| 100,000 | 10 | 2.619 | 1.372 | 7.29M | 1002.505 | 730.49× | 19.87× |
| 100,000 | 1,000 | 33.747 | 34.942 | 28.62M | 1027.670 | 29.41× | 1.07× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 85.08M | 85.95M | 1.00× | 2.13M | 2.63M | 1.00× | 78.47M |
| 2 | 165.64M | 173.15M | 2.01× | 2.24M | 2.47M | 0.94× | 82.74M |
| 4 | 305.35M | 310.41M | 3.61× | 2.24M | 2.49M | 0.95× | 81.33M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

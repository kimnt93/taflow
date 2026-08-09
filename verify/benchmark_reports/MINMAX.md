# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.15M | 0.006 | 169.52M | 0.040 | 5.73× | 6.74× |
| 10,000 | 0.067 | 148.21M | 0.061 | 162.94M | 0.115 | 1.70× | 1.87× |
| 100,000 | 0.699 | 143.03M | 0.644 | 155.32M | 0.806 | 1.15× | 1.25× |
| 1,000,000 | 8.093 | 123.57M | 7.224 | 138.42M | 7.978 | 0.99× | 1.10× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.708 ms**; native kernel **0.633 ms**; TA-Lib 0.806 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.310 | 0.247 | 4.04M | 793.960 | 3210.71× | 135.33× |
| 100,000 | 10 | 1.759 | 1.457 | 6.86M | 807.242 | 553.89× | 22.61× |
| 100,000 | 1,000 | 83.416 | 68.017 | 14.70M | 803.917 | 11.82× | 0.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 102.58M | 119.10M | 1.00× | 2.00M | 2.15M | 1.00× | 92.81M |
| 2 | 179.04M | 238.75M | 2.00× | 2.21M | 2.37M | 1.10× | 100.88M |
| 4 | 226.89M | 280.18M | 2.35× | 2.14M | 2.23M | 1.04× | 100.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

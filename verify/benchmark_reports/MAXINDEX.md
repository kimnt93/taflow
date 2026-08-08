# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.76M | 0.011 | 88.33M | 0.037 | 3.34× | 3.25× |
| 10,000 | 0.153 | 65.21M | 0.151 | 66.14M | 0.103 | 0.67× | 0.68× |
| 100,000 | 1.539 | 64.98M | 1.509 | 66.29M | 0.703 | 0.46× | 0.47× |
| 1,000,000 | 15.567 | 64.24M | 14.929 | 66.99M | 7.032 | 0.45× | 0.47× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.550 ms**; native kernel **1.522 ms**; TA-Lib 0.705 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.253 | 0.174 | 5.75M | 705.523 | 4057.24× | 168.50× |
| 100,000 | 10 | 1.030 | 0.705 | 14.18M | 701.875 | 995.42× | 42.69× |
| 100,000 | 1,000 | 17.250 | 16.375 | 61.07M | 709.662 | 43.34× | 2.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 56.19M | 60.35M | 1.00× | 2.61M | 3.19M | 1.00× | 107.57M |
| 2 | 102.46M | 116.01M | 1.92× | 2.87M | 3.35M | 1.05× | 116.81M |
| 4 | 199.00M | 229.19M | 3.80× | 2.87M | 2.99M | 0.94× | 119.68M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

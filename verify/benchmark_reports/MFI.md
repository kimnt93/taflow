# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.40M | 0.010 | 97.48M | 0.035 | 2.85× | 3.46× |
| 10,000 | 0.179 | 55.78M | 0.191 | 52.27M | 0.108 | 0.60× | 0.57× |
| 100,000 | 1.782 | 56.12M | 1.774 | 56.39M | 0.875 | 0.49× | 0.49× |
| 1,000,000 | 18.780 | 53.25M | 17.561 | 56.95M | 8.682 | 0.46× | 0.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.751 ms**; native kernel **1.739 ms**; TA-Lib 0.852 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.417 | 0.255 | 3.92M | 886.508 | 3473.12× | 118.50× |
| 100,000 | 10 | 2.457 | 1.356 | 7.38M | 873.523 | 644.30× | 22.74× |
| 100,000 | 1,000 | 24.364 | 19.411 | 51.52M | 878.990 | 45.28× | 1.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 51.15M | 53.56M | 1.00× | 1.95M | 2.42M | 1.00× | 97.80M |
| 2 | 97.90M | 102.71M | 1.92× | 1.75M | 2.62M | 1.08× | 90.51M |
| 4 | 164.20M | 194.47M | 3.63× | 1.81M | 2.15M | 0.89× | 93.28M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

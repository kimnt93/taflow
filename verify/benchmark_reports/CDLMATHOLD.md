# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.76M | 0.014 | 70.23M | 0.040 | 2.42× | 2.84× |
| 10,000 | 0.169 | 59.04M | 0.166 | 60.06M | 0.117 | 0.69× | 0.70× |
| 100,000 | 1.716 | 58.27M | 1.695 | 59.00M | 0.858 | 0.50× | 0.51× |
| 1,000,000 | 18.151 | 55.09M | 17.288 | 57.84M | 8.540 | 0.47× | 0.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.683 ms**; native kernel **1.688 ms**; TA-Lib 0.885 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.264 | 3.78M | 866.641 | 3276.89× | 120.09× |
| 100,000 | 10 | 2.599 | 1.201 | 8.33M | 873.267 | 727.41× | 26.47× |
| 100,000 | 1,000 | 36.186 | 21.557 | 46.39M | 900.783 | 41.79× | 1.79× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 52.74M | 56.42M | 1.00× | 2.05M | 2.19M | 1.00× | 101.58M |
| 2 | 54.60M | 55.14M | 0.98× | 2.18M | 2.42M | 1.11× | 97.80M |
| 4 | 53.56M | 54.27M | 0.96× | 2.21M | 2.34M | 1.07× | 96.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

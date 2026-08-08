# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.59M | 0.004 | 234.34M | 0.036 | 5.72× | 8.40× |
| 10,000 | 0.053 | 187.10M | 0.050 | 199.72M | 0.109 | 2.04× | 2.18× |
| 100,000 | 0.650 | 153.85M | 0.626 | 159.85M | 0.844 | 1.30× | 1.35× |
| 1,000,000 | 6.736 | 148.45M | 6.659 | 150.17M | 9.086 | 1.35× | 1.36× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.654 ms**; native kernel **0.641 ms**; TA-Lib 0.853 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.350 | 0.280 | 3.57M | 828.103 | 2958.35× | 100.98× |
| 100,000 | 10 | 2.709 | 1.440 | 6.94M | 873.967 | 606.88× | 20.05× |
| 100,000 | 1,000 | 31.615 | 25.412 | 39.35M | 829.163 | 32.63× | 1.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 109.24M | 140.91M | 1.00× | 2.13M | 2.03M | 1.00× | 99.29M |
| 2 | 259.08M | 267.09M | 1.90× | 2.46M | 2.45M | 1.21× | 100.66M |
| 4 | 404.69M | 444.95M | 3.16× | 2.50M | 2.58M | 1.27× | 104.43M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

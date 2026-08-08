# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.66M | 0.005 | 197.56M | 0.038 | 5.50× | 7.52× |
| 10,000 | 0.055 | 183.03M | 0.048 | 209.66M | 0.101 | 1.84× | 2.11× |
| 100,000 | 0.557 | 179.54M | 0.535 | 186.90M | 0.688 | 1.24× | 1.29× |
| 1,000,000 | 6.256 | 159.84M | 6.112 | 163.62M | 7.262 | 1.16× | 1.19× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.562 ms**; native kernel **0.534 ms**; TA-Lib 0.721 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.379 | 0.305 | 3.28M | 736.918 | 2415.06× | 100.32× |
| 100,000 | 10 | 2.981 | 1.477 | 6.77M | 711.790 | 481.89× | 19.98× |
| 100,000 | 1,000 | 23.908 | 20.310 | 49.24M | 687.882 | 33.87× | 1.73× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 145.82M | 156.02M | 1.00× | 2.49M | 2.76M | 1.00× | 131.24M |
| 2 | 278.25M | 293.42M | 1.88× | 2.29M | 2.72M | 0.99× | 117.56M |
| 4 | 455.99M | 473.40M | 3.03× | 2.18M | 2.75M | 1.00× | 125.30M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

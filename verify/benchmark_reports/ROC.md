# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 256.24M | 0.003 | 350.39M | 0.031 | 7.96× | 10.88× |
| 10,000 | 0.023 | 438.07M | 0.020 | 495.37M | 0.040 | 1.75× | 1.97× |
| 100,000 | 0.209 | 478.20M | 0.185 | 541.55M | 0.122 | 0.58× | 0.66× |
| 1,000,000 | 2.299 | 435.03M | 1.906 | 524.57M | 1.104 | 0.48× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.209 ms**; native kernel **0.183 ms**; TA-Lib 0.121 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.249 | 0.142 | 7.02M | 121.240 | 851.56× | 198.32× |
| 100,000 | 10 | 0.848 | 0.484 | 20.65M | 122.591 | 253.12× | 57.86× |
| 100,000 | 1,000 | 4.225 | 3.202 | 312.26M | 121.822 | 38.04× | 9.65× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 279.66M | 310.71M | 1.00× | 3.53M | 3.75M | 1.00× | 419.85M |
| 2 | 503.34M | 601.79M | 1.94× | 3.19M | 4.20M | 1.12× | 444.60M |
| 4 | 650.91M | 1.14G | 3.66× | 3.35M | 3.50M | 0.93× | 483.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.20M | 0.004 | 254.29M | 0.032 | 6.82× | 8.21× |
| 10,000 | 0.021 | 473.71M | 0.018 | 541.31M | 0.042 | 1.99× | 2.27× |
| 100,000 | 0.190 | 527.26M | 0.161 | 622.31M | 0.126 | 0.66× | 0.78× |
| 1,000,000 | 2.175 | 459.72M | 1.618 | 618.06M | 1.087 | 0.50× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.122 | 1.59× |
| 1 | 5 | 0.268 | 0.518 | 1.93× |
| 1 | 10 | 0.500 | 1.009 | 2.02× |
| 10 | 1 | 0.051 | 0.097 | 1.93× |
| 10 | 5 | 0.225 | 0.454 | 2.02× |
| 10 | 10 | 0.476 | 0.938 | 1.97× |
| 100 | 1 | 0.049 | 0.094 | 1.92× |
| 100 | 5 | 0.217 | 0.440 | 2.03× |
| 100 | 10 | 0.472 | 0.948 | 2.01× |
| 1,000 | 1 | 0.049 | 0.092 | 1.89× |
| 1,000 | 5 | 0.235 | 0.448 | 1.91× |
| 1,000 | 10 | 0.476 | 0.975 | 2.05× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.156 | 6.41M | 131.586 | 843.44× | 192.30× |
| 100,000 | 10 | 0.839 | 0.476 | 21.00M | 127.999 | 268.82× | 62.87× |
| 100,000 | 1,000 | 7.330 | 3.274 | 305.45M | 125.167 | 38.23× | 12.65× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 286.30M | 404.52M | 1.00× | 3.26M | 3.73M | 1.00× | 437.80M |
| 5 | 610.72M | 832.75M | 2.06× | 3.04M | 2.99M | 0.80× | 420.33M |
| 10 | 593.66M | 1.01G | 2.50× | 2.55M | 2.70M | 0.72× | 392.76M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

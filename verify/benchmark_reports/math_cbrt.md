# MathCbrt benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.75M | 0.019 | 51.50M | 0.016 | 0.75× | 0.80× |
| 10,000 | 0.186 | 53.64M | 0.168 | 59.68M | 0.145 | 0.78× | 0.87× |
| 100,000 | 1.716 | 58.26M | 1.666 | 60.04M | 1.393 | 0.81× | 0.84× |
| 1,000,000 | 17.165 | 58.26M | 17.637 | 56.70M | 18.131 | 1.06× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.058 | 0.43× |
| 1 | 5 | 0.302 | 0.189 | 0.63× |
| 1 | 10 | 0.486 | 0.406 | 0.84× |
| 10 | 1 | 0.047 | 0.041 | 0.87× |
| 10 | 5 | 0.236 | 0.189 | 0.80× |
| 10 | 10 | 0.538 | 0.443 | 0.82× |
| 100 | 1 | 0.054 | 0.048 | 0.90× |
| 100 | 5 | 0.238 | 0.234 | 0.98× |
| 100 | 10 | 0.503 | 0.420 | 0.83× |
| 1,000 | 1 | 0.066 | 0.071 | 1.06× |
| 1,000 | 5 | 0.232 | 0.224 | 0.96× |
| 1,000 | 10 | 0.517 | 0.457 | 0.88× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.315 | 0.173 | 5.80M | nan | — | — |
| 100,000 | 10 | 1.151 | 0.670 | 14.93M | nan | — | — |
| 100,000 | 1,000 | 20.678 | 18.326 | 54.57M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 50.14M | 53.29M | 1.00× | 2.62M | 3.60M | 1.00× | — |
| 5 | 181.11M | 209.13M | 3.92× | 2.00M | 2.59M | 0.72× | — |
| 10 | 245.87M | 286.57M | 5.38× | 1.98M | 2.52M | 0.70× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# Ichimoku benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.091 | 10.98M | 0.092 | 10.84M | nan | — | — |
| 10,000 | 0.965 | 10.37M | 0.912 | 10.96M | nan | — | — |
| 100,000 | 9.367 | 10.68M | 9.260 | 10.80M | nan | — | — |
| 1,000,000 | 114.129 | 8.76M | 100.509 | 9.95M | nan | — | — |

## Warm-up

Construct + canonical extend over 100,000 bars: **9.618 ms**; native kernel **9.252 ms**.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.447 | 0.387 | 2.58M | nan | — | — |
| 100,000 | 10 | 3.351 | 2.130 | 4.69M | nan | — | — |
| 100,000 | 1,000 | 103.903 | 104.261 | 9.59M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.16M | 9.90M | 1.00× | 1.59M | 1.51M | 1.00× | — |
| 2 | 19.26M | 20.38M | 2.06× | 1.75M | 1.72M | 1.14× | — |
| 4 | 33.17M | 33.38M | 3.37× | 1.75M | 1.62M | 1.07× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

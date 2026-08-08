# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 194.00M | 0.004 | 248.69M | 0.028 | 5.38× | 6.90× |
| 10,000 | 0.036 | 275.41M | 0.034 | 293.51M | 0.055 | 1.51× | 1.61× |
| 100,000 | 0.349 | 286.66M | 0.322 | 310.18M | 0.290 | 0.83× | 0.90× |
| 1,000,000 | 4.409 | 226.80M | 3.916 | 255.38M | 2.771 | 0.63× | 0.71× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.339 ms**; native kernel **0.317 ms**; TA-Lib 0.289 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.221 | 0.152 | 6.57M | 284.301 | 1869.21× | 163.22× |
| 100,000 | 10 | 0.900 | 0.553 | 18.10M | 288.738 | 522.50× | 44.94× |
| 100,000 | 1,000 | 5.688 | 4.556 | 219.48M | 288.296 | 63.28× | 6.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 160.30M | 196.66M | 1.00× | 2.42M | 2.97M | 1.00× | 230.41M |
| 2 | 267.39M | 409.97M | 2.08× | 3.08M | 3.37M | 1.14× | 231.88M |
| 4 | 405.90M | 645.82M | 3.28× | 3.06M | 2.94M | 0.99× | 257.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

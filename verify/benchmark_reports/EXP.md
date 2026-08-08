# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.98M | 0.006 | 159.88M | 0.029 | 3.94× | 4.63× |
| 10,000 | 0.059 | 169.44M | 0.058 | 173.29M | 0.072 | 1.23× | 1.25× |
| 100,000 | 0.582 | 171.70M | 0.557 | 179.67M | 0.468 | 0.80× | 0.84× |
| 1,000,000 | 6.394 | 156.39M | 6.043 | 165.49M | 4.472 | 0.70× | 0.74× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.563 ms**; native kernel **0.541 ms**; TA-Lib 0.468 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.164 | 6.10M | 465.938 | 2842.62× | 149.95× |
| 100,000 | 10 | 0.969 | 0.587 | 17.02M | 467.480 | 795.76× | 41.81× |
| 100,000 | 1,000 | 8.025 | 9.907 | 100.94M | 479.279 | 48.38× | 3.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.57M | 124.64M | 1.00× | 2.70M | 2.98M | 1.00× | 163.82M |
| 2 | 242.76M | 242.68M | 1.95× | 2.99M | 3.42M | 1.15× | 168.06M |
| 4 | 344.30M | 368.13M | 2.95× | 2.83M | 3.41M | 1.14× | 173.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

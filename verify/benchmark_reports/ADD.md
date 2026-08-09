# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 218.43M | 0.003 | 297.48M | 0.032 | 6.93× | 9.44× |
| 10,000 | 0.011 | 899.96M | 0.011 | 937.64M | 0.034 | 3.08× | 3.21× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.005 ms**; native kernel **0.004 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.337 | 0.193 | 5.18M | 28.349 | 146.80× | 148.42× |
| 1,500 | 10 | 1.650 | 0.776 | 12.89M | 29.079 | 37.48× | 35.63× |
| 1,500 | 100 | 3.390 | 1.775 | 56.35M | 28.844 | 16.25× | 16.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.71M | 14.23M | 1.00× | 1.04M | 1.37M | 1.00× | 9.36M |
| 2 | 16.97M | 21.96M | 1.54× | 1.38M | 1.68M | 1.22× | 10.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

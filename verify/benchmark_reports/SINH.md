# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.56M | 0.009 | 117.57M | 0.035 | 3.54× | 4.10× |
| 10,000 | 0.065 | 153.43M | 0.063 | 158.51M | 0.087 | 1.34× | 1.39× |
| 100,000 | 0.607 | 164.70M | 0.587 | 170.41M | 0.639 | 1.05× | 1.09× |
| 1,000,000 | 6.276 | 159.35M | 6.015 | 166.24M | 6.242 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.123 | 1.47× |
| 1 | 5 | 0.308 | 0.461 | 1.50× |
| 1 | 10 | 0.454 | 0.896 | 1.97× |
| 10 | 1 | 0.052 | 0.085 | 1.63× |
| 10 | 5 | 0.228 | 0.434 | 1.91× |
| 10 | 10 | 0.464 | 0.883 | 1.90× |
| 100 | 1 | 0.047 | 0.087 | 1.84× |
| 100 | 5 | 0.219 | 0.412 | 1.88× |
| 100 | 10 | 0.491 | 0.907 | 1.85× |
| 1,000 | 1 | 0.055 | 0.092 | 1.66× |
| 1,000 | 5 | 0.251 | 0.473 | 1.88× |
| 1,000 | 10 | 0.476 | 0.934 | 1.96× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.312 | 0.163 | 6.15M | 639.613 | 3931.64× | 158.77× |
| 100,000 | 10 | 1.214 | 0.556 | 18.00M | 651.933 | 1173.19× | 47.68× |
| 100,000 | 1,000 | 9.610 | 8.223 | 121.60M | 677.759 | 82.42× | 4.13× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 131.90M | 136.64M | 1.00× | 2.56M | 3.68M | 1.00× | 123.84M |
| 5 | 350.63M | 555.22M | 4.06× | 2.08M | 2.80M | 0.76× | 129.48M |
| 10 | 438.23M | 586.00M | 4.29× | 2.04M | 2.60M | 0.71× | 127.95M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

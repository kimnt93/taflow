# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 105.91M | 0.009 | 112.25M | 0.033 | 3.54× | 3.76× |
| 10,000 | 0.077 | 129.80M | 0.074 | 135.62M | 0.094 | 1.23× | 1.28× |
| 100,000 | 0.736 | 135.88M | 0.694 | 144.07M | 0.662 | 0.90× | 0.95× |
| 1,000,000 | 7.890 | 126.74M | 7.635 | 130.98M | 6.474 | 0.82× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.741 ms**; native kernel **0.702 ms**; TA-Lib 0.660 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.246 | 0.163 | 6.13M | 662.837 | 4062.40× | 152.04× |
| 100,000 | 10 | 0.997 | 0.622 | 16.08M | 672.123 | 1080.56× | 40.55× |
| 100,000 | 1,000 | 13.804 | 9.044 | 110.57M | 674.328 | 74.56× | 3.68× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 114.97M | 103.64M | 1.00× | 3.10M | 3.61M | 1.00× | 132.80M |
| 2 | 210.36M | 215.58M | 2.08× | 2.99M | 3.44M | 0.95× | 126.41M |
| 4 | 314.85M | 381.24M | 3.68× | 2.75M | 2.97M | 0.82× | 131.80M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

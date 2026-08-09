# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.20M | 0.010 | 101.50M | 0.046 | 4.12× | 4.69× |
| 10,000 | 0.089 | 112.76M | 0.081 | 123.98M | 0.107 | 1.21× | 1.33× |
| 100,000 | 0.877 | 113.99M | 0.807 | 123.94M | 0.745 | 0.85× | 0.92× |
| 1,000,000 | 18.290 | 54.68M | 14.535 | 68.80M | 11.700 | 0.64× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.879 ms**; native kernel **0.789 ms**; TA-Lib 0.748 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.369 | 0.335 | 2.98M | 731.790 | 2184.29× | 120.73× |
| 100,000 | 10 | 2.027 | 2.159 | 4.63M | 737.734 | 341.63× | 18.26× |
| 100,000 | 1,000 | 92.240 | 77.939 | 12.83M | 739.163 | 9.48× | 0.63× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 63.26M | 108.31M | 1.00× | 1.97M | 1.72M | 1.00× | 105.39M |
| 2 | 122.64M | 184.50M | 1.70× | 1.79M | 1.75M | 1.02× | 104.57M |
| 4 | 175.92M | 343.13M | 3.17× | 1.38M | 1.44M | 0.84× | 100.23M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

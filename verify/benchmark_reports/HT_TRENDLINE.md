# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.074 | 13.45M | 0.074 | 13.59M | 0.085 | 1.15× | 1.16× |
| 10,000 | 0.731 | 13.69M | 0.759 | 13.17M | 0.642 | 0.88× | 0.85× |
| 100,000 | 7.230 | 13.83M | 7.468 | 13.39M | 6.116 | 0.85× | 0.82× |
| 1,000,000 | 76.066 | 13.15M | 74.409 | 13.44M | 60.960 | 0.80× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.178 | 1.58× |
| 1 | 5 | 0.320 | 0.536 | 1.68× |
| 1 | 10 | 0.473 | 0.903 | 1.91× |
| 10 | 1 | 0.050 | 0.091 | 1.80× |
| 10 | 5 | 0.264 | 0.494 | 1.87× |
| 10 | 10 | 0.510 | 0.954 | 1.87× |
| 100 | 1 | 0.058 | 0.097 | 1.67× |
| 100 | 5 | 0.241 | 0.517 | 2.14× |
| 100 | 10 | 0.517 | 1.021 | 1.97× |
| 1,000 | 1 | 0.128 | 0.152 | 1.19× |
| 1,000 | 5 | 0.310 | 0.805 | 2.60× |
| 1,000 | 10 | 0.628 | 1.657 | 2.64× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.408 | 0.303 | 3.30M | 6029.253 | 19891.09× | 103.60× |
| 100,000 | 10 | 2.570 | 2.060 | 4.85M | 6088.064 | 2955.04× | 14.60× |
| 100,000 | 1,000 | 128.699 | 132.566 | 7.54M | 5939.044 | 44.80× | 0.66× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.03M | 13.46M | 1.00× | 1.97M | 2.17M | 1.00× | 16.18M |
| 5 | 58.28M | 55.31M | 4.11× | 2.25M | 2.29M | 1.06× | 15.25M |
| 10 | 90.79M | 96.45M | 7.16× | 2.21M | 2.14M | 0.99× | 15.61M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

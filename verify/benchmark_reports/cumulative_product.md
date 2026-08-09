# CumulativeProduct benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 267.78M | 0.003 | 345.07M | 0.053 | 14.18× | 18.28× |
| 10,000 | 0.014 | 729.04M | 0.012 | 866.17M | 0.086 | 6.27× | 7.45× |
| 100,000 | 0.115 | 869.68M | 0.095 | 1.05G | 0.439 | 3.82× | 4.60× |
| 1,000,000 | 1.447 | 691.21M | 1.005 | 994.72M | 3.963 | 2.74× | 3.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.189 | 1.17× |
| 1 | 5 | 0.304 | 0.657 | 2.16× |
| 1 | 10 | 0.476 | 1.288 | 2.71× |
| 10 | 1 | 0.050 | 0.153 | 3.03× |
| 10 | 5 | 0.231 | 0.575 | 2.49× |
| 10 | 10 | 0.518 | 1.655 | 3.19× |
| 100 | 1 | 0.051 | 0.162 | 3.17× |
| 100 | 5 | 0.232 | 0.591 | 2.55× |
| 100 | 10 | 0.482 | 1.203 | 2.49× |
| 1,000 | 1 | 0.052 | 0.162 | 3.13× |
| 1,000 | 5 | 0.230 | 0.570 | 2.48× |
| 1,000 | 10 | 0.480 | 1.177 | 2.45× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.152 | 6.58M | nan | — | — |
| 100,000 | 10 | 0.900 | 0.528 | 18.92M | nan | — | — |
| 100,000 | 1,000 | 3.351 | 2.555 | 391.39M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 521.30M | 639.08M | 1.00× | 3.59M | 4.20M | 1.00× | — |
| 5 | 774.25M | 1.64G | 2.56× | 3.00M | 3.36M | 0.80× | — |
| 10 | 648.75M | 1.43G | 2.24× | 2.88M | 3.29M | 0.78× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

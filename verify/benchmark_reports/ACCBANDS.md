# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.54M | 0.013 | 77.91M | 0.051 | 3.71× | 3.99× |
| 10,000 | 0.098 | 102.49M | 0.091 | 109.50M | 0.123 | 1.26× | 1.34× |
| 100,000 | 1.168 | 85.64M | 0.890 | 112.34M | 0.839 | 0.72× | 0.94× |
| 1,000,000 | 19.347 | 51.69M | 17.970 | 55.65M | 14.281 | 0.74× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.161 | 1.94× |
| 1 | 5 | 0.344 | 0.579 | 1.68× |
| 1 | 10 | 0.550 | 1.165 | 2.12× |
| 10 | 1 | 0.054 | 0.107 | 1.99× |
| 10 | 5 | 0.241 | 0.514 | 2.14× |
| 10 | 10 | 0.493 | 1.163 | 2.36× |
| 100 | 1 | 0.071 | 0.118 | 1.67× |
| 100 | 5 | 0.273 | 0.616 | 2.26× |
| 100 | 10 | 0.529 | 1.154 | 2.18× |
| 1,000 | 1 | 0.070 | 0.118 | 1.69× |
| 1,000 | 5 | 0.292 | 0.611 | 2.09× |
| 1,000 | 10 | 0.560 | 1.202 | 2.15× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.558 | 0.467 | 2.14M | 1062.026 | 2271.99× | 104.50× |
| 100,000 | 10 | 3.276 | 2.294 | 4.36M | 821.850 | 358.27× | 20.56× |
| 100,000 | 1,000 | 110.791 | 119.597 | 8.36M | 836.034 | 6.99× | 0.48× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 67.01M | 103.69M | 1.00× | 1.61M | 1.51M | 1.00× | 91.17M |
| 5 | 151.96M | 267.16M | 2.58× | 1.01M | 944.50K | 0.62× | 97.12M |
| 10 | 155.31M | 272.12M | 2.62× | 1.15M | 1.18M | 0.78× | 97.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

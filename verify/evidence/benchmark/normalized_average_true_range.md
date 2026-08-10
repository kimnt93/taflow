# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.56M | 0.012 | 81.08M | 0.041 | 2.42× | 3.30× |
| 10,000 | 0.082 | 122.69M | 0.077 | 129.84M | 0.095 | 1.16× | 1.23× |
| 100,000 | 0.790 | 126.65M | 0.740 | 135.20M | 0.649 | 0.82× | 0.88× |
| 1,000,000 | 8.600 | 116.28M | 8.742 | 114.39M | 7.873 | 0.92× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.137 | 1.88× |
| 1 | 5 | 0.295 | 0.508 | 1.72× |
| 1 | 10 | 0.569 | 0.973 | 1.71× |
| 10 | 1 | 0.053 | 0.095 | 1.80× |
| 10 | 5 | 0.227 | 0.465 | 2.05× |
| 10 | 10 | 0.624 | 1.212 | 1.94× |
| 100 | 1 | 0.055 | 0.096 | 1.73× |
| 100 | 5 | 0.242 | 0.452 | 1.87× |
| 100 | 10 | 0.502 | 1.059 | 2.11× |
| 1,000 | 1 | 0.060 | 0.102 | 1.71× |
| 1,000 | 5 | 0.268 | 0.490 | 1.83× |
| 1,000 | 10 | 0.536 | 1.080 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

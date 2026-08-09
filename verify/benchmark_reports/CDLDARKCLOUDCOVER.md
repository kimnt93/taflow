# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.95M | 0.007 | 148.61M | 0.034 | 4.00× | 5.08× |
| 10,000 | 0.056 | 179.73M | 0.054 | 184.59M | 0.108 | 1.93× | 1.99× |
| 100,000 | 0.827 | 120.92M | 0.818 | 122.20M | 0.805 | 0.97× | 0.98× |
| 1,000,000 | 8.404 | 118.99M | 8.633 | 115.84M | 7.873 | 0.94× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.147 | 1.25× |
| 1 | 5 | 0.332 | 0.562 | 1.69× |
| 1 | 10 | 0.506 | 0.967 | 1.91× |
| 10 | 1 | 0.051 | 0.094 | 1.85× |
| 10 | 5 | 0.245 | 0.453 | 1.85× |
| 10 | 10 | 0.498 | 0.943 | 1.90× |
| 100 | 1 | 0.055 | 0.097 | 1.78× |
| 100 | 5 | 0.239 | 0.455 | 1.90× |
| 100 | 10 | 0.524 | 0.968 | 1.85× |
| 1,000 | 1 | 0.065 | 0.113 | 1.74× |
| 1,000 | 5 | 0.251 | 0.496 | 1.97× |
| 1,000 | 10 | 0.544 | 1.070 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

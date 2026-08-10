# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.88M | 0.018 | 55.02M | 0.044 | 2.00× | 2.40× |
| 10,000 | 0.152 | 65.67M | 0.150 | 66.73M | 0.126 | 0.83× | 0.84× |
| 100,000 | 1.559 | 64.14M | 1.492 | 67.00M | 1.027 | 0.66× | 0.69× |
| 1,000,000 | 15.427 | 64.82M | 15.286 | 65.42M | 8.848 | 0.57× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.142 | 1.10× |
| 1 | 5 | 0.375 | 0.565 | 1.50× |
| 1 | 10 | 0.584 | 1.129 | 1.94× |
| 10 | 1 | 0.057 | 0.095 | 1.67× |
| 10 | 5 | 0.365 | 0.577 | 1.58× |
| 10 | 10 | 0.606 | 1.080 | 1.78× |
| 100 | 1 | 0.075 | 0.107 | 1.43× |
| 100 | 5 | 0.320 | 0.867 | 2.71× |
| 100 | 10 | 0.625 | 1.091 | 1.75× |
| 1,000 | 1 | 0.081 | 0.118 | 1.45× |
| 1,000 | 5 | 0.332 | 0.717 | 2.16× |
| 1,000 | 10 | 0.740 | 1.302 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

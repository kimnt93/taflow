# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.72M | 0.018 | 55.56M | 0.042 | 2.00× | 2.33× |
| 10,000 | 0.141 | 70.72M | 0.139 | 71.91M | 0.100 | 0.71× | 0.72× |
| 100,000 | 1.264 | 79.13M | 1.232 | 81.17M | 0.703 | 0.56× | 0.57× |
| 1,000,000 | 12.850 | 77.82M | 12.650 | 79.05M | 7.029 | 0.55× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.148 | 1.72× |
| 1 | 5 | 0.473 | 0.566 | 1.20× |
| 1 | 10 | 0.652 | 1.115 | 1.71× |
| 10 | 1 | 0.059 | 0.101 | 1.73× |
| 10 | 5 | 0.310 | 0.533 | 1.72× |
| 10 | 10 | 0.686 | 1.213 | 1.77× |
| 100 | 1 | 0.112 | 0.122 | 1.09× |
| 100 | 5 | 0.346 | 0.657 | 1.90× |
| 100 | 10 | 0.672 | 1.173 | 1.75× |
| 1,000 | 1 | 0.098 | 0.115 | 1.17× |
| 1,000 | 5 | 0.345 | 0.659 | 1.91× |
| 1,000 | 10 | 0.660 | 1.219 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.82M | 0.008 | 130.74M | 0.055 | 5.94× | 7.13× |
| 10,000 | 0.068 | 146.42M | 0.063 | 158.78M | 0.117 | 1.72× | 1.86× |
| 100,000 | 1.650 | 60.62M | 0.930 | 107.53M | 0.852 | 0.52× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.196 | 2.34× |
| 1 | 5 | 0.322 | 0.594 | 1.84× |
| 1 | 10 | 0.396 | 1.089 | 2.75× |
| 10 | 1 | 0.043 | 0.104 | 2.44× |
| 10 | 5 | 0.190 | 0.563 | 2.97× |
| 10 | 10 | 0.390 | 1.110 | 2.85× |
| 100 | 1 | 0.044 | 0.112 | 2.55× |
| 100 | 5 | 0.197 | 0.529 | 2.69× |
| 100 | 10 | 0.418 | 1.156 | 2.76× |
| 1,000 | 1 | 0.058 | 0.111 | 1.90× |
| 1,000 | 5 | 0.210 | 0.556 | 2.65× |
| 1,000 | 10 | 0.439 | 1.268 | 2.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.73M | 0.017 | 59.36M | 0.041 | 2.07× | 2.43× |
| 10,000 | 0.173 | 57.76M | 0.167 | 59.99M | 0.173 | 1.00× | 1.04× |
| 100,000 | 1.688 | 59.24M | 1.670 | 59.87M | 1.503 | 0.89× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.153 | 1.57× |
| 1 | 5 | 0.312 | 0.520 | 1.67× |
| 1 | 10 | 0.543 | 0.902 | 1.66× |
| 10 | 1 | 0.057 | 0.088 | 1.54× |
| 10 | 5 | 0.239 | 0.479 | 2.01× |
| 10 | 10 | 0.588 | 0.925 | 1.57× |
| 100 | 1 | 0.059 | 0.089 | 1.51× |
| 100 | 5 | 0.260 | 0.432 | 1.66× |
| 100 | 10 | 0.570 | 0.952 | 1.67× |
| 1,000 | 1 | 0.078 | 0.115 | 1.46× |
| 1,000 | 5 | 0.254 | 0.530 | 2.08× |
| 1,000 | 10 | 0.603 | 1.170 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

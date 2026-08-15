# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.07M | 0.006 | 172.47M | 0.043 | 6.25× | 7.38× |
| 10,000 | 0.050 | 201.07M | 0.043 | 230.05M | 0.118 | 2.37× | 2.71× |
| 100,000 | 0.454 | 220.07M | 0.416 | 240.55M | 0.849 | 1.87× | 2.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.129 | 1.00× |
| 1 | 5 | 0.257 | 0.480 | 1.86× |
| 1 | 10 | 0.428 | 1.042 | 2.44× |
| 10 | 1 | 0.046 | 0.096 | 2.07× |
| 10 | 5 | 0.195 | 0.477 | 2.44× |
| 10 | 10 | 0.402 | 1.092 | 2.72× |
| 100 | 1 | 0.046 | 0.106 | 2.30× |
| 100 | 5 | 0.184 | 0.453 | 2.47× |
| 100 | 10 | 0.413 | 1.015 | 2.46× |
| 1,000 | 1 | 0.055 | 0.103 | 1.88× |
| 1,000 | 5 | 0.219 | 0.533 | 2.43× |
| 1,000 | 10 | 0.422 | 1.097 | 2.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

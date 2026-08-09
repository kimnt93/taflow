# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.60M | 0.005 | 200.54M | 0.049 | 6.93× | 9.75× |
| 10,000 | 0.039 | 257.60M | 0.035 | 286.64M | 0.085 | 2.18× | 2.42× |
| 100,000 | 0.383 | 260.91M | 0.354 | 282.77M | 0.588 | 1.53× | 1.66× |
| 1,000,000 | 4.050 | 246.92M | 3.807 | 262.66M | 5.140 | 1.27× | 1.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.152 | 0.94× |
| 1 | 5 | 0.257 | 0.497 | 1.93× |
| 1 | 10 | 0.534 | 1.108 | 2.07× |
| 10 | 1 | 0.063 | 0.107 | 1.71× |
| 10 | 5 | 0.257 | 0.507 | 1.97× |
| 10 | 10 | 0.536 | 1.113 | 2.08× |
| 100 | 1 | 0.065 | 0.102 | 1.57× |
| 100 | 5 | 0.289 | 0.526 | 1.82× |
| 100 | 10 | 0.529 | 1.139 | 2.15× |
| 1,000 | 1 | 0.057 | 0.108 | 1.88× |
| 1,000 | 5 | 0.264 | 0.573 | 2.17× |
| 1,000 | 10 | 0.591 | 1.097 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

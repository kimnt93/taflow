# MoneyFlowIndex benchmark (`MFI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.83M | 0.011 | 87.56M | 0.038 | 2.74× | 3.34× |
| 10,000 | 0.066 | 151.08M | 0.063 | 159.75M | 0.113 | 1.71× | 1.81× |
| 100,000 | 0.636 | 157.25M | 0.586 | 170.53M | 0.918 | 1.44× | 1.56× |
| 1,000,000 | 7.895 | 126.66M | 7.144 | 139.98M | 11.316 | 1.43× | 1.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.177 | 1.86× |
| 1 | 5 | 0.721 | 1.123 | 1.56× |
| 1 | 10 | 0.647 | 1.187 | 1.84× |
| 10 | 1 | 0.056 | 0.111 | 1.99× |
| 10 | 5 | 0.328 | 0.674 | 2.05× |
| 10 | 10 | 0.548 | 1.069 | 1.95× |
| 100 | 1 | 0.058 | 0.102 | 1.75× |
| 100 | 5 | 0.309 | 0.577 | 1.87× |
| 100 | 10 | 0.580 | 1.104 | 1.90× |
| 1,000 | 1 | 0.063 | 0.117 | 1.87× |
| 1,000 | 5 | 0.302 | 0.602 | 1.99× |
| 1,000 | 10 | 0.653 | 1.302 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.83M | 0.035 | 28.50M | 0.245 | 6.32× | 6.97× |
| 10,000 | 0.348 | 28.71M | 0.368 | 27.21M | 0.860 | 2.47× | 2.34× |
| 100,000 | 3.611 | 27.69M | 3.513 | 28.47M | 7.111 | 1.97× | 2.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.291 | 5.12× |
| 1 | 5 | 0.271 | 1.354 | 5.00× |
| 1 | 10 | 0.420 | 2.710 | 6.46× |
| 10 | 1 | 0.048 | 0.245 | 5.08× |
| 10 | 5 | 0.203 | 1.508 | 7.44× |
| 10 | 10 | 0.412 | 2.532 | 6.15× |
| 100 | 1 | 0.049 | 0.272 | 5.58× |
| 100 | 5 | 0.238 | 1.537 | 6.47× |
| 100 | 10 | 0.415 | 2.865 | 6.91× |
| 1,000 | 1 | 0.092 | 0.321 | 3.50× |
| 1,000 | 5 | 0.257 | 1.727 | 6.73× |
| 1,000 | 10 | 0.455 | 3.359 | 7.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

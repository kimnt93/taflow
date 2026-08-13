# StandardErrorBands benchmark (`StandardErrorBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.933 | 1.07M | 0.937 | 1.07M | 0.601 | 0.64× | 0.64× |
| 10,000 | 9.342 | 1.07M | 9.309 | 1.07M | 4.153 | 0.44× | 0.45× |
| 100,000 | 94.099 | 1.06M | 98.821 | 1.01M | 44.636 | 0.47× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.311 | 3.25× |
| 1 | 5 | 0.395 | 1.423 | 3.60× |
| 1 | 10 | 0.599 | 2.567 | 4.29× |
| 10 | 1 | 0.079 | 0.254 | 3.23× |
| 10 | 5 | 0.314 | 1.420 | 4.52× |
| 10 | 10 | 0.631 | 2.728 | 4.32× |
| 100 | 1 | 0.160 | 0.293 | 1.83× |
| 100 | 5 | 0.314 | 1.614 | 5.13× |
| 100 | 10 | 0.682 | 3.033 | 4.45× |
| 1,000 | 1 | 1.044 | 0.895 | 0.86× |
| 1,000 | 5 | 1.480 | 3.743 | 2.53× |
| 1,000 | 10 | 2.182 | 7.452 | 3.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

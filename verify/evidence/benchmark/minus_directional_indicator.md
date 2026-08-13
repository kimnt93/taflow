# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.81M | 0.077 | 12.92M | 0.039 | 0.46× | 0.50× |
| 10,000 | 0.690 | 14.49M | 0.674 | 14.83M | 0.098 | 0.14× | 0.15× |
| 100,000 | 6.900 | 14.49M | 6.669 | 14.99M | 0.652 | 0.09× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.114 | 0.68× |
| 1 | 5 | 0.347 | 0.496 | 1.43× |
| 1 | 10 | 0.635 | 0.957 | 1.51× |
| 10 | 1 | 0.068 | 0.100 | 1.48× |
| 10 | 5 | 0.310 | 0.459 | 1.48× |
| 10 | 10 | 0.645 | 0.952 | 1.48× |
| 100 | 1 | 0.074 | 0.099 | 1.33× |
| 100 | 5 | 0.309 | 0.462 | 1.50× |
| 100 | 10 | 0.661 | 0.943 | 1.43× |
| 1,000 | 1 | 0.139 | 0.098 | 0.70× |
| 1,000 | 5 | 0.328 | 0.478 | 1.46× |
| 1,000 | 10 | 0.674 | 1.044 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

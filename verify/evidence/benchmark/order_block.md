# OrderBlock benchmark (`causal dual-scale order blocks` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.078 | 12.81M | 0.073 | 13.70M | 9.731 | 124.63× | 133.33× |
| 10,000 | 0.873 | 11.46M | 0.795 | 12.58M | 117.992 | 135.16× | 148.46× |
| 100,000 | 8.841 | 11.31M | 8.659 | 11.55M | 1286.732 | 145.54× | 148.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.262 | 2.81× |
| 1 | 5 | 0.271 | 0.844 | 3.12× |
| 1 | 10 | 0.447 | 1.642 | 3.67× |
| 10 | 1 | 0.047 | 0.174 | 3.70× |
| 10 | 5 | 0.210 | 0.879 | 4.18× |
| 10 | 10 | 0.435 | 1.755 | 4.04× |
| 100 | 1 | 0.060 | 0.647 | 10.83× |
| 100 | 5 | 0.221 | 3.276 | 14.83× |
| 100 | 10 | 0.472 | 6.537 | 13.85× |
| 1,000 | 1 | 0.128 | 9.843 | 76.78× |
| 1,000 | 5 | 0.458 | 50.705 | 110.60× |
| 1,000 | 10 | 0.603 | 257.789 | 427.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

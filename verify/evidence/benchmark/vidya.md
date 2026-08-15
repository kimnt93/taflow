# VariableIndexDynamicAverage benchmark (`VIDYA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.11M | 0.011 | 87.21M | 0.216 | 17.11× | 18.86× |
| 10,000 | 0.134 | 74.81M | 0.163 | 61.45M | 0.596 | 4.46× | 3.66× |
| 100,000 | 1.142 | 87.57M | 1.097 | 91.13M | 3.932 | 3.44× | 3.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.299 | 5.49× |
| 1 | 5 | 0.248 | 1.455 | 5.86× |
| 1 | 10 | 0.385 | 2.986 | 7.75× |
| 10 | 1 | 0.046 | 0.271 | 5.87× |
| 10 | 5 | 0.185 | 1.574 | 8.50× |
| 10 | 10 | 0.443 | 2.688 | 6.07× |
| 100 | 1 | 0.043 | 0.255 | 5.96× |
| 100 | 5 | 0.221 | 1.624 | 7.35× |
| 100 | 10 | 0.415 | 2.933 | 7.07× |
| 1,000 | 1 | 0.066 | 0.308 | 4.70× |
| 1,000 | 5 | 0.198 | 1.722 | 8.68× |
| 1,000 | 10 | 0.436 | 3.211 | 7.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

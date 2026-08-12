# OrderBlock benchmark (`causal dual-scale order blocks` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.090 | 11.14M | 0.106 | 9.42M | 9.915 | 110.46× | 93.36× |
| 10,000 | 0.847 | 11.80M | 0.824 | 12.14M | 118.495 | 139.88× | 143.87× |
| 100,000 | 9.312 | 10.74M | 9.064 | 11.03M | 1311.136 | 140.80× | 144.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.186 | 0.224 | 1.20× |
| 1 | 5 | 0.314 | 0.881 | 2.80× |
| 1 | 10 | 0.529 | 1.828 | 3.46× |
| 10 | 1 | 0.064 | 0.187 | 2.95× |
| 10 | 5 | 0.271 | 0.955 | 3.52× |
| 10 | 10 | 0.563 | 1.886 | 3.35× |
| 100 | 1 | 0.068 | 0.639 | 9.45× |
| 100 | 5 | 0.284 | 3.214 | 11.31× |
| 100 | 10 | 0.611 | 6.431 | 10.52× |
| 1,000 | 1 | 0.145 | 9.640 | 66.44× |
| 1,000 | 5 | 0.426 | 50.490 | 118.63× |
| 1,000 | 10 | 0.754 | 100.656 | 133.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

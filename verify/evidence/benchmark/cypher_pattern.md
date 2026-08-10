# CypherPattern benchmark (`Cypher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.98M | 0.010 | 100.45M | 0.221 | 17.23× | 22.20× |
| 10,000 | 0.086 | 116.60M | 0.085 | 117.57M | 1.460 | 17.02× | 17.16× |
| 100,000 | 0.816 | 122.48M | 0.785 | 127.35M | 12.364 | 15.14× | 15.75× |
| 1,000,000 | 8.359 | 119.63M | 7.890 | 126.74M | 134.522 | 16.09× | 17.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.224 | 2.90× |
| 1 | 5 | 0.271 | 2.245 | 8.28× |
| 1 | 10 | 0.825 | 2.494 | 3.02× |
| 10 | 1 | 0.091 | 0.252 | 2.77× |
| 10 | 5 | 0.361 | 1.690 | 4.69× |
| 10 | 10 | 0.810 | 2.477 | 3.06× |
| 100 | 1 | 0.087 | 0.296 | 3.38× |
| 100 | 5 | 0.382 | 1.377 | 3.61× |
| 100 | 10 | 0.677 | 2.066 | 3.05× |
| 1,000 | 1 | 0.087 | 0.392 | 4.52× |
| 1,000 | 5 | 0.356 | 2.068 | 5.81× |
| 1,000 | 10 | 0.659 | 3.205 | 4.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

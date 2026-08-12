# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.50M | 0.010 | 105.23M | 0.159 | 15.30× | 16.69× |
| 10,000 | 0.071 | 141.61M | 0.070 | 142.14M | 0.486 | 6.88× | 6.90× |
| 100,000 | 0.684 | 146.16M | 0.678 | 147.39M | 3.779 | 5.52× | 5.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.235 | 1.79× |
| 1 | 5 | 0.253 | 0.985 | 3.89× |
| 1 | 10 | 0.467 | 2.082 | 4.46× |
| 10 | 1 | 0.051 | 0.198 | 3.89× |
| 10 | 5 | 0.237 | 0.970 | 4.10× |
| 10 | 10 | 0.494 | 2.126 | 4.30× |
| 100 | 1 | 0.058 | 0.207 | 3.57× |
| 100 | 5 | 0.233 | 0.978 | 4.20× |
| 100 | 10 | 0.485 | 2.229 | 4.60× |
| 1,000 | 1 | 0.065 | 0.240 | 3.67× |
| 1,000 | 5 | 0.238 | 1.162 | 4.88× |
| 1,000 | 10 | 0.530 | 2.667 | 5.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

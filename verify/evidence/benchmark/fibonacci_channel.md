# FibonacciChannel benchmark (`FibChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.104 | 9.59M | 0.090 | 11.09M | 0.512 | 4.91× | 5.68× |
| 10,000 | 0.867 | 11.53M | 0.842 | 11.88M | 3.936 | 4.54× | 4.68× |
| 100,000 | 8.429 | 11.86M | 8.990 | 11.12M | 45.109 | 5.35× | 5.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.206 | 1.70× |
| 1 | 5 | 0.335 | 0.864 | 2.58× |
| 1 | 10 | 0.614 | 1.852 | 3.02× |
| 10 | 1 | 0.071 | 0.172 | 2.43× |
| 10 | 5 | 0.306 | 0.843 | 2.75× |
| 10 | 10 | 0.611 | 1.889 | 3.09× |
| 100 | 1 | 0.079 | 0.208 | 2.63× |
| 100 | 5 | 0.312 | 1.044 | 3.35× |
| 100 | 10 | 0.647 | 2.270 | 3.51× |
| 1,000 | 1 | 0.159 | 0.799 | 5.02× |
| 1,000 | 5 | 0.333 | 3.260 | 9.79× |
| 1,000 | 10 | 0.656 | 12.920 | 19.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

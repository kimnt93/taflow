# FibonacciFan benchmark (`FibFan` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.69M | 0.078 | 12.90M | 0.516 | 6.03× | 6.65× |
| 10,000 | 0.695 | 14.40M | 0.680 | 14.71M | 3.874 | 5.58× | 5.70× |
| 100,000 | 7.036 | 14.21M | 6.791 | 14.73M | 41.346 | 5.88× | 6.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.231 | 2.44× |
| 1 | 5 | 0.390 | 0.857 | 2.20× |
| 1 | 10 | 0.610 | 1.852 | 3.04× |
| 10 | 1 | 0.071 | 0.170 | 2.40× |
| 10 | 5 | 0.292 | 0.835 | 2.86× |
| 10 | 10 | 0.620 | 1.924 | 3.10× |
| 100 | 1 | 0.078 | 0.208 | 2.66× |
| 100 | 5 | 0.324 | 1.019 | 3.14× |
| 100 | 10 | 0.642 | 2.319 | 3.61× |
| 1,000 | 1 | 0.151 | 0.772 | 5.11× |
| 1,000 | 5 | 0.330 | 3.126 | 9.46× |
| 1,000 | 10 | 0.670 | 6.336 | 9.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

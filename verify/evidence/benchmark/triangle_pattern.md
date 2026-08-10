# TrianglePattern benchmark (`Triangle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.87M | 0.012 | 84.46M | 0.227 | 15.18× | 19.17× |
| 10,000 | 0.100 | 100.30M | 0.101 | 99.10M | 1.302 | 13.06× | 12.90× |
| 100,000 | 0.971 | 103.00M | 0.952 | 105.06M | 12.635 | 13.01× | 13.27× |
| 1,000,000 | 9.708 | 103.01M | 9.353 | 106.91M | 124.609 | 12.84× | 13.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.203 | 2.24× |
| 1 | 5 | 0.288 | 1.072 | 3.72× |
| 1 | 10 | 0.518 | 1.661 | 3.21× |
| 10 | 1 | 0.062 | 0.169 | 2.73× |
| 10 | 5 | 0.242 | 1.078 | 4.46× |
| 10 | 10 | 0.507 | 1.697 | 3.35× |
| 100 | 1 | 0.060 | 0.176 | 2.92× |
| 100 | 5 | 0.266 | 1.131 | 4.25× |
| 100 | 10 | 0.561 | 1.855 | 3.31× |
| 1,000 | 1 | 0.070 | 0.302 | 4.33× |
| 1,000 | 5 | 0.258 | 1.730 | 6.70× |
| 1,000 | 10 | 0.548 | 3.005 | 5.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

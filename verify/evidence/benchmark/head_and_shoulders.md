# HeadAndShoulders benchmark (`HeadAndShoulders` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.72M | 0.007 | 136.75M | 0.234 | 23.35× | 32.01× |
| 10,000 | 0.096 | 103.99M | 0.106 | 94.43M | 1.502 | 15.62× | 14.19× |
| 100,000 | 0.937 | 106.70M | 0.897 | 111.52M | 12.639 | 13.49× | 14.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.208 | 2.07× |
| 1 | 5 | 0.244 | 0.858 | 3.52× |
| 1 | 10 | 0.410 | 1.653 | 4.03× |
| 10 | 1 | 0.043 | 0.161 | 3.75× |
| 10 | 5 | 0.199 | 1.071 | 5.37× |
| 10 | 10 | 0.402 | 1.722 | 4.28× |
| 100 | 1 | 0.045 | 0.179 | 3.99× |
| 100 | 5 | 0.197 | 1.141 | 5.79× |
| 100 | 10 | 0.427 | 1.863 | 4.37× |
| 1,000 | 1 | 0.058 | 0.300 | 5.13× |
| 1,000 | 5 | 0.193 | 1.742 | 9.01× |
| 1,000 | 10 | 0.436 | 3.005 | 6.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

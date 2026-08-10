# FourPointHarmonicPattern benchmark (`Abcd` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.41M | 0.012 | 81.05M | 0.239 | 16.11× | 19.37× |
| 10,000 | 0.100 | 99.58M | 0.095 | 105.62M | 1.492 | 14.86× | 15.76× |
| 100,000 | 0.943 | 106.06M | 0.919 | 108.78M | 13.709 | 14.54× | 14.91× |
| 1,000,000 | 9.854 | 101.48M | 8.852 | 112.97M | 138.422 | 14.05× | 15.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.224 | 1.91× |
| 1 | 5 | 0.298 | 1.093 | 3.66× |
| 1 | 10 | 0.524 | 1.674 | 3.20× |
| 10 | 1 | 0.058 | 0.167 | 2.88× |
| 10 | 5 | 0.241 | 1.090 | 4.53× |
| 10 | 10 | 0.510 | 1.697 | 3.33× |
| 100 | 1 | 0.060 | 0.182 | 3.04× |
| 100 | 5 | 0.252 | 1.113 | 4.41× |
| 100 | 10 | 0.527 | 1.811 | 3.44× |
| 1,000 | 1 | 0.064 | 0.298 | 4.62× |
| 1,000 | 5 | 0.266 | 1.745 | 6.55× |
| 1,000 | 10 | 0.560 | 3.030 | 5.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

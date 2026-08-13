# SmoothedTrendChannel benchmark (`smoothed trend channel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.085 | 11.82M | 0.076 | 13.24M | 0.570 | 6.74× | 7.55× |
| 10,000 | 0.670 | 14.92M | 0.661 | 15.14M | 5.003 | 7.46× | 7.57× |
| 100,000 | 6.631 | 15.08M | 6.308 | 15.85M | 49.475 | 7.46× | 7.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.179 | 1.51× |
| 1 | 5 | 0.435 | 0.544 | 1.25× |
| 1 | 10 | 0.628 | 1.038 | 1.65× |
| 10 | 1 | 0.071 | 0.184 | 2.59× |
| 10 | 5 | 0.313 | 0.852 | 2.72× |
| 10 | 10 | 0.633 | 1.714 | 2.71× |
| 100 | 1 | 0.076 | 0.208 | 2.72× |
| 100 | 5 | 0.321 | 1.081 | 3.36× |
| 100 | 10 | 0.659 | 2.149 | 3.26× |
| 1,000 | 1 | 0.144 | 0.674 | 4.67× |
| 1,000 | 5 | 0.344 | 3.390 | 9.84× |
| 1,000 | 10 | 0.699 | 6.800 | 9.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

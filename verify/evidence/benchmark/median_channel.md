# MedianChannel benchmark (`MedianChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.159 | 6.28M | 0.157 | 6.36M | 0.925 | 5.81× | 5.89× |
| 10,000 | 1.641 | 6.10M | 1.680 | 5.95M | 8.194 | 4.99× | 4.88× |
| 100,000 | 17.056 | 5.86M | 16.409 | 6.09M | 80.792 | 4.74× | 4.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.296 | 4.79× |
| 1 | 5 | 0.211 | 1.430 | 6.78× |
| 1 | 10 | 0.422 | 2.548 | 6.03× |
| 10 | 1 | 0.048 | 0.246 | 5.16× |
| 10 | 5 | 0.217 | 1.471 | 6.77× |
| 10 | 10 | 0.423 | 2.858 | 6.75× |
| 100 | 1 | 0.064 | 0.324 | 5.04× |
| 100 | 5 | 0.205 | 1.794 | 8.73× |
| 100 | 10 | 0.460 | 3.463 | 7.52× |
| 1,000 | 1 | 0.222 | 1.237 | 5.57× |
| 1,000 | 5 | 0.407 | 5.597 | 13.74× |
| 1,000 | 10 | 0.618 | 11.636 | 18.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

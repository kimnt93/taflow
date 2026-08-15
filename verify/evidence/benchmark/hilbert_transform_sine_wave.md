# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.129 | 7.76M | 0.128 | 7.80M | 0.489 | 3.79× | 3.81× |
| 10,000 | 1.375 | 7.27M | 1.346 | 7.43M | 4.545 | 3.31× | 3.38× |
| 100,000 | 13.831 | 7.23M | 13.649 | 7.33M | 45.331 | 3.28× | 3.32× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.179 | 0.154 | 0.86× |
| 1 | 5 | 0.286 | 0.593 | 2.08× |
| 1 | 10 | 0.387 | 0.914 | 2.36× |
| 10 | 1 | 0.044 | 0.088 | 2.03× |
| 10 | 5 | 0.181 | 0.472 | 2.61× |
| 10 | 10 | 0.424 | 0.940 | 2.22× |
| 100 | 1 | 0.053 | 0.117 | 2.20× |
| 100 | 5 | 0.202 | 0.577 | 2.86× |
| 100 | 10 | 0.428 | 1.226 | 2.87× |
| 1,000 | 1 | 0.185 | 0.549 | 2.98× |
| 1,000 | 5 | 0.300 | 2.723 | 9.08× |
| 1,000 | 10 | 0.536 | 5.722 | 10.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

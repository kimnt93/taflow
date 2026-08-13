# NegativeVolumeIndex benchmark (`NVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 26.90M | 0.031 | 32.73M | 0.195 | 5.24× | 6.38× |
| 10,000 | 0.317 | 31.52M | 0.233 | 42.90M | 0.773 | 2.44× | 3.32× |
| 100,000 | 2.268 | 44.09M | 2.253 | 44.39M | 6.423 | 2.83× | 2.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.266 | 1.66× |
| 1 | 5 | 0.400 | 1.021 | 2.55× |
| 1 | 10 | 0.599 | 2.155 | 3.60× |
| 10 | 1 | 0.071 | 0.199 | 2.81× |
| 10 | 5 | 0.296 | 1.283 | 4.33× |
| 10 | 10 | 0.618 | 2.267 | 3.67× |
| 100 | 1 | 0.074 | 0.211 | 2.86× |
| 100 | 5 | 0.300 | 1.327 | 4.42× |
| 100 | 10 | 0.566 | 2.267 | 4.00× |
| 1,000 | 1 | 0.097 | 0.262 | 2.69× |
| 1,000 | 5 | 0.304 | 1.583 | 5.21× |
| 1,000 | 10 | 0.611 | 2.877 | 4.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

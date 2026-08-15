# AutomaticFibonacci benchmark (`AutoFib` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 36.85M | 0.021 | 48.01M | 0.707 | 26.06× | 33.96× |
| 10,000 | 0.234 | 42.78M | 0.206 | 48.54M | 5.543 | 23.72× | 26.91× |
| 100,000 | 2.457 | 40.70M | 2.065 | 48.42M | 65.689 | 26.73× | 31.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.203 | 2.76× |
| 1 | 5 | 0.291 | 0.830 | 2.85× |
| 1 | 10 | 0.417 | 1.882 | 4.51× |
| 10 | 1 | 0.043 | 0.172 | 3.96× |
| 10 | 5 | 0.196 | 0.851 | 4.34× |
| 10 | 10 | 0.443 | 1.964 | 4.43× |
| 100 | 1 | 0.052 | 0.229 | 4.41× |
| 100 | 5 | 0.189 | 1.117 | 5.92× |
| 100 | 10 | 0.456 | 2.427 | 5.32× |
| 1,000 | 1 | 0.076 | 0.956 | 12.59× |
| 1,000 | 5 | 0.218 | 4.336 | 19.87× |
| 1,000 | 10 | 0.474 | 8.428 | 17.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

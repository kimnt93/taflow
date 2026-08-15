# SignedPower benchmark (`numpy.sign/abs/power` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 260.18M | 0.003 | 346.84M | 0.027 | 7.11× | 9.48× |
| 10,000 | 0.022 | 464.37M | 0.019 | 524.01M | 0.048 | 2.25× | 2.53× |
| 100,000 | 0.190 | 526.14M | 0.169 | 590.52M | 0.210 | 1.10× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.100 | 0.80× |
| 1 | 5 | 0.221 | 0.423 | 1.92× |
| 1 | 10 | 0.408 | 0.852 | 2.09× |
| 10 | 1 | 0.039 | 0.094 | 2.38× |
| 10 | 5 | 0.195 | 0.415 | 2.13× |
| 10 | 10 | 0.386 | 0.899 | 2.33× |
| 100 | 1 | 0.045 | 0.092 | 2.03× |
| 100 | 5 | 0.180 | 0.416 | 2.31× |
| 100 | 10 | 0.404 | 0.855 | 2.12× |
| 1,000 | 1 | 0.043 | 0.091 | 2.14× |
| 1,000 | 5 | 0.208 | 0.509 | 2.44× |
| 1,000 | 10 | 0.460 | 1.035 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

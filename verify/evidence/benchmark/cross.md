# Cross benchmark (`causal cross event` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.43M | 0.005 | 190.21M | 0.021 | 3.11× | 4.06× |
| 10,000 | 0.055 | 183.18M | 0.050 | 200.10M | 0.048 | 0.87× | 0.95× |
| 100,000 | 0.469 | 213.03M | 0.450 | 222.38M | 0.298 | 0.64× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.101 | 1.30× |
| 1 | 5 | 0.320 | 0.348 | 1.09× |
| 1 | 10 | 0.403 | 0.752 | 1.86× |
| 10 | 1 | 0.043 | 0.072 | 1.66× |
| 10 | 5 | 0.178 | 0.341 | 1.92× |
| 10 | 10 | 0.407 | 0.732 | 1.80× |
| 100 | 1 | 0.045 | 0.069 | 1.53× |
| 100 | 5 | 0.195 | 0.357 | 1.83× |
| 100 | 10 | 0.433 | 0.743 | 1.72× |
| 1,000 | 1 | 0.053 | 0.074 | 1.42× |
| 1,000 | 5 | 0.203 | 0.683 | 3.36× |
| 1,000 | 10 | 0.442 | 1.206 | 2.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

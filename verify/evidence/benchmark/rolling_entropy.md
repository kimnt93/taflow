# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 8.890 | 112.49K | 9.409 | 106.28K | 0.049 | 0.01× | 0.01× |
| 10,000 | 91.439 | 109.36K | 91.987 | 108.71K | 0.115 | 0.00× | 0.00× |
| 100,000 | 918.906 | 108.83K | 957.490 | 104.44K | 0.960 | 0.00× | 0.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.113 | 0.76× |
| 1 | 5 | 0.403 | 0.416 | 1.03× |
| 1 | 10 | 0.647 | 0.803 | 1.24× |
| 10 | 1 | 0.071 | 0.082 | 1.15× |
| 10 | 5 | 0.303 | 0.389 | 1.28× |
| 10 | 10 | 0.658 | 0.832 | 1.26× |
| 100 | 1 | 0.889 | 0.115 | 0.13× |
| 100 | 5 | 1.465 | 0.557 | 0.38× |
| 100 | 10 | 1.877 | 1.122 | 0.60× |
| 1,000 | 1 | 9.665 | 0.180 | 0.02× |
| 1,000 | 5 | 12.694 | 0.736 | 0.06× |
| 1,000 | 10 | 17.506 | 1.555 | 0.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

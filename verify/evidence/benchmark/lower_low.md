# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 370.34M | 0.001 | 771.42M | 0.025 | 9.30× | 19.38× |
| 10,000 | 0.009 | 1.07G | 0.005 | 1.83G | 0.048 | 5.14× | 8.74× |
| 100,000 | 0.081 | 1.23G | 0.055 | 1.82G | 0.268 | 3.29× | 4.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.101 | 0.74× |
| 1 | 5 | 0.304 | 0.354 | 1.17× |
| 1 | 10 | 0.430 | 0.831 | 1.94× |
| 10 | 1 | 0.046 | 0.073 | 1.60× |
| 10 | 5 | 0.188 | 0.373 | 1.99× |
| 10 | 10 | 0.415 | 0.782 | 1.89× |
| 100 | 1 | 0.045 | 0.098 | 2.21× |
| 100 | 5 | 0.209 | 0.419 | 2.01× |
| 100 | 10 | 0.416 | 0.780 | 1.87× |
| 1,000 | 1 | 0.041 | 0.084 | 2.04× |
| 1,000 | 5 | 0.207 | 0.527 | 2.54× |
| 1,000 | 10 | 0.471 | 1.252 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

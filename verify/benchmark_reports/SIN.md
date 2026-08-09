# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.02M | 0.012 | 82.45M | 0.036 | 2.71× | 2.94× |
| 10,000 | 0.151 | 66.33M | 0.149 | 67.12M | 0.180 | 1.19× | 1.21× |
| 100,000 | 1.567 | 63.83M | 1.577 | 63.43M | 1.670 | 1.07× | 1.06× |
| 1,000,000 | 17.480 | 57.21M | 16.738 | 59.74M | 15.981 | 0.91× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.116 | 1.54× |
| 1 | 5 | 0.257 | 0.496 | 1.93× |
| 1 | 10 | 0.509 | 1.034 | 2.03× |
| 10 | 1 | 0.055 | 0.092 | 1.66× |
| 10 | 5 | 0.246 | 0.461 | 1.88× |
| 10 | 10 | 0.517 | 0.992 | 1.92× |
| 100 | 1 | 0.056 | 0.090 | 1.60× |
| 100 | 5 | 0.262 | 0.480 | 1.83× |
| 100 | 10 | 0.568 | 1.024 | 1.80× |
| 1,000 | 1 | 0.071 | 0.117 | 1.64× |
| 1,000 | 5 | 0.277 | 0.549 | 1.98× |
| 1,000 | 10 | 0.556 | 1.241 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.68M | 0.016 | 61.99M | 0.088 | 5.01× | 5.48× |
| 10,000 | 0.139 | 71.90M | 0.132 | 75.56M | 0.632 | 4.55× | 4.78× |
| 100,000 | 1.278 | 78.24M | 1.324 | 75.51M | 5.911 | 4.63× | 4.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.144 | 1.60× |
| 1 | 5 | 0.415 | 0.512 | 1.23× |
| 1 | 10 | 0.505 | 0.914 | 1.81× |
| 10 | 1 | 0.049 | 0.092 | 1.87× |
| 10 | 5 | 0.221 | 0.477 | 2.16× |
| 10 | 10 | 0.506 | 1.010 | 2.00× |
| 100 | 1 | 0.057 | 0.098 | 1.71× |
| 100 | 5 | 0.247 | 0.468 | 1.90× |
| 100 | 10 | 0.462 | 1.043 | 2.26× |
| 1,000 | 1 | 0.080 | 0.159 | 2.00× |
| 1,000 | 5 | 0.247 | 0.770 | 3.11× |
| 1,000 | 10 | 0.521 | 1.564 | 3.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

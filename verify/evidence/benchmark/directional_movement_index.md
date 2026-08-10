# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.67M | 0.016 | 64.32M | 0.043 | 2.29× | 2.79× |
| 10,000 | 0.153 | 65.42M | 0.112 | 89.00M | 0.122 | 0.80× | 1.08× |
| 100,000 | 1.063 | 94.06M | 1.037 | 96.46M | 0.905 | 0.85× | 0.87× |
| 1,000,000 | 11.117 | 89.95M | 10.684 | 93.60M | 9.246 | 0.83× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.130 | 1.21× |
| 1 | 5 | 0.350 | 0.512 | 1.46× |
| 1 | 10 | 0.524 | 1.020 | 1.95× |
| 10 | 1 | 0.071 | 0.104 | 1.46× |
| 10 | 5 | 0.262 | 0.489 | 1.86× |
| 10 | 10 | 0.547 | 0.965 | 1.77× |
| 100 | 1 | 0.056 | 0.097 | 1.74× |
| 100 | 5 | 0.262 | 0.482 | 1.84× |
| 100 | 10 | 0.592 | 1.043 | 1.76× |
| 1,000 | 1 | 0.068 | 0.107 | 1.58× |
| 1,000 | 5 | 0.264 | 0.514 | 1.94× |
| 1,000 | 10 | 0.594 | 1.106 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

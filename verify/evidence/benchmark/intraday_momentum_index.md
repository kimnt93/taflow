# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.82M | 0.008 | 118.29M | 0.091 | 7.90× | 10.76× |
| 10,000 | 0.053 | 189.91M | 0.054 | 183.54M | 0.623 | 11.83× | 11.43× |
| 100,000 | 0.512 | 195.23M | 0.487 | 205.17M | 6.012 | 11.74× | 12.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.144 | 1.21× |
| 1 | 5 | 0.216 | 0.495 | 2.29× |
| 1 | 10 | 0.405 | 0.996 | 2.46× |
| 10 | 1 | 0.042 | 0.093 | 2.22× |
| 10 | 5 | 0.176 | 0.448 | 2.55× |
| 10 | 10 | 0.397 | 1.074 | 2.71× |
| 100 | 1 | 0.052 | 0.113 | 2.19× |
| 100 | 5 | 0.191 | 0.472 | 2.47× |
| 100 | 10 | 0.407 | 1.002 | 2.46× |
| 1,000 | 1 | 0.059 | 0.155 | 2.64× |
| 1,000 | 5 | 0.234 | 0.818 | 3.49× |
| 1,000 | 10 | 0.431 | 1.541 | 3.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

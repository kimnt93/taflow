# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.08M | 0.002 | 471.27M | 0.027 | 0.60× | 12.88× |
| 10,000 | 0.430 | 23.24M | 0.013 | 784.44M | 0.035 | 0.08× | 2.78× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.065 ms**; native kernel **0.003 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.705 | 0.253 | 3.95M | 28.324 | 111.97× | 101.06× |
| 1,500 | 10 | 2.652 | 1.023 | 9.77M | 27.961 | 27.33× | 25.50× |
| 1,500 | 100 | 9.798 | 2.605 | 38.39M | 29.406 | 11.29× | 9.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

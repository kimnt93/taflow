# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.06M | 0.028 | 35.19M | 0.036 | 0.54× | 1.26× |
| 10,000 | 0.662 | 15.11M | 0.250 | 40.08M | 0.083 | 0.12× | 0.33× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.099 ms**; native kernel **0.029 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.321 | 0.208 | 4.81M | 40.198 | 193.55× | 155.37× |
| 1,500 | 10 | 2.045 | 0.945 | 10.58M | 40.490 | 42.84× | 35.60× |
| 1,500 | 100 | 9.523 | 4.472 | 22.36M | 52.020 | 11.63× | 6.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

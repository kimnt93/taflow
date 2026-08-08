# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.483 | 2.07M | 0.459 | 2.18M | 0.499 | 1.03× | 1.09× |
| 10,000 | 4.597 | 2.18M | 4.683 | 2.14M | 4.982 | 1.08× | 1.06× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.706 ms**; native kernel **0.665 ms**; TA-Lib 0.722 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.959 | 0.709 | 1.41M | 722.955 | 1019.77× | 54.81× |
| 1,500 | 10 | 10.327 | 5.696 | 1.76M | 715.047 | 125.53× | 7.73× |
| 1,500 | 100 | 51.449 | 49.561 | 2.02M | 787.143 | 15.88× | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.

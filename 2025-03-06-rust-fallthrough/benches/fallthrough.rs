// benches/break.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(always)]
fn rotl32(x: u32, r: u32) -> u32 {
    x.rotate_left(r)
}

#[inline(never)]
fn break_version(
    len: u32,
    mut k1: u32,
    mut k2: u32,
    mut k3: u32,
    mut k4: u32,
    mut h1: u32,
    mut h2: u32,
    mut h3: u32,
    mut h4: u32,
    c1: u32,
    c2: u32,
    c3: u32,
    c4: u32,
    tail: &[u8],
) -> u32 {
    // rem = len & 15
    let rem = (len & 15) as usize;
    'outer: {
        'case1: {
            'case2: {
                'case3: {
                    'case4: {
                        'case5: {
                            'case6: {
                                'case7: {
                                    'case8: {
                                        'case9: {
                                            'case10: {
                                                'case11: {
                                                    'case12: {
                                                        'case13: {
                                                            'case14: {
                                                                'case15: {
                                                                    // The match "jumps" to the appropriate label,
                                                                    // skipping the code for all earlier cases.
                                                                    match rem {
                                                                        15 => break 'case15,
                                                                        14 => break 'case14,
                                                                        13 => break 'case13,
                                                                        12 => break 'case12,
                                                                        11 => break 'case11,
                                                                        10 => break 'case10,
                                                                        9  => break 'case9,
                                                                        8  => break 'case8,
                                                                        7  => break 'case7,
                                                                        6  => break 'case6,
                                                                        5  => break 'case5,
                                                                        4  => break 'case4,
                                                                        3  => break 'case3,
                                                                        2  => break 'case2,
                                                                        1  => break 'case1,
                                                                        _  => break 'outer,
                                                                    }
                                                                } // end 'case15:
                                                                k4 ^= (tail[14] as u32) << 16;
                                                            } // end 'case14:
                                                            k4 ^= (tail[13] as u32) << 8;
                                                        } // end 'case13:
                                                        k4 ^= (tail[12] as u32) << 0;
                                                        k4 = k4.wrapping_mul(c4);
                                                        k4 = rotl32(k4, 18);
                                                        k4 = k4.wrapping_mul(c1);
                                                        h4 ^= k4;
                                                    } // end 'case12:
                                                    k3 ^= (tail[11] as u32) << 24;
                                                } // end 'case11:
                                                k3 ^= (tail[10] as u32) << 16;
                                            } // end 'case10:
                                            k3 ^= (tail[9] as u32) << 8;
                                        } // end 'case9:
                                        k3 ^= (tail[8] as u32) << 0;
                                        k3 = k3.wrapping_mul(c3);
                                        k3 = rotl32(k3, 17);
                                        k3 = k3.wrapping_mul(c4);
                                        h3 ^= k3;
                                    } // end 'case8:
                                    k2 ^= (tail[7] as u32) << 24;
                                } // end 'case7:
                                k2 ^= (tail[6] as u32) << 16;
                            } // end 'case6:
                            k2 ^= (tail[5] as u32) << 8;
                        } // end 'case5:
                        k2 ^= (tail[4] as u32) << 0;
                        k2 = k2.wrapping_mul(c2);
                        k2 = rotl32(k2, 16);
                        k2 = k2.wrapping_mul(c3);
                        h2 ^= k2;
                    } // end 'case4:
                    k1 ^= (tail[3] as u32) << 24;
                } // end 'case3:
                k1 ^= (tail[2] as u32) << 16;
            } // end 'case2:
            k1 ^= (tail[1] as u32) << 8;
        } // end 'case1:
        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = rotl32(k1, 15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    } // end 'outer:
    h1.wrapping_add(h2).wrapping_add(h3).wrapping_add(h4)
}

/// This version implements the same logic using a cascade of if–statements.
#[inline(never)]
fn if_version(
    len: u32,
    mut k1: u32,
    mut k2: u32,
    mut k3: u32,
    mut k4: u32,
    mut h1: u32,
    mut h2: u32,
    mut h3: u32,
    mut h4: u32,
    c1: u32,
    c2: u32,
    c3: u32,
    c4: u32,
    tail: &[u8],
) -> u32 {
    let rem = (len & 15) as usize;
    if rem >= 15 {
        k4 ^= (tail[14] as u32) << 16;
    }
    if rem >= 14 {
        k4 ^= (tail[13] as u32) << 8;
    }
    if rem >= 13 {
        k4 ^= (tail[12] as u32) << 0;
        k4 = k4.wrapping_mul(c4);
        k4 = rotl32(k4, 18);
        k4 = k4.wrapping_mul(c1);
        h4 ^= k4;
    }
    if rem >= 12 {
        k3 ^= (tail[11] as u32) << 24;
    }
    if rem >= 11 {
        k3 ^= (tail[10] as u32) << 16;
    }
    if rem >= 10 {
        k3 ^= (tail[9] as u32) << 8;
    }
    if rem >= 9 {
        k3 ^= (tail[8] as u32) << 0;
        k3 = k3.wrapping_mul(c3);
        k3 = rotl32(k3, 17);
        k3 = k3.wrapping_mul(c4);
        h3 ^= k3;
    }
    if rem >= 8 {
        k2 ^= (tail[7] as u32) << 24;
    }
    if rem >= 7 {
        k2 ^= (tail[6] as u32) << 16;
    }
    if rem >= 6 {
        k2 ^= (tail[5] as u32) << 8;
    }
    if rem >= 5 {
        k2 ^= (tail[4] as u32) << 0;
        k2 = k2.wrapping_mul(c2);
        k2 = rotl32(k2, 16);
        k2 = k2.wrapping_mul(c3);
        h2 ^= k2;
    }
    if rem >= 4 {
        k1 ^= (tail[3] as u32) << 24;
    }
    if rem >= 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if rem >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if rem >= 1 {
        k1 ^= (tail[0] as u32) << 0;
        k1 = k1.wrapping_mul(c1);
        k1 = rotl32(k1, 15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }
    h1.wrapping_add(h2).wrapping_add(h3).wrapping_add(h4)
}

#[inline(always)]
fn break_version_inline(
    len: u32,
    mut k1: u32,
    mut k2: u32,
    mut k3: u32,
    mut k4: u32,
    mut h1: u32,
    mut h2: u32,
    mut h3: u32,
    mut h4: u32,
    c1: u32,
    c2: u32,
    c3: u32,
    c4: u32,
    tail: &[u8],
) -> u32 {
    // rem = len & 15
    let rem = (len & 15) as usize;
    'outer: {
        'case1: {
            'case2: {
                'case3: {
                    'case4: {
                        'case5: {
                            'case6: {
                                'case7: {
                                    'case8: {
                                        'case9: {
                                            'case10: {
                                                'case11: {
                                                    'case12: {
                                                        'case13: {
                                                            'case14: {
                                                                'case15: {
                                                                    // The match "jumps" to the appropriate label,
                                                                    // skipping the code for all earlier cases.
                                                                    match rem {
                                                                        15 => break 'case15,
                                                                        14 => break 'case14,
                                                                        13 => break 'case13,
                                                                        12 => break 'case12,
                                                                        11 => break 'case11,
                                                                        10 => break 'case10,
                                                                        9  => break 'case9,
                                                                        8  => break 'case8,
                                                                        7  => break 'case7,
                                                                        6  => break 'case6,
                                                                        5  => break 'case5,
                                                                        4  => break 'case4,
                                                                        3  => break 'case3,
                                                                        2  => break 'case2,
                                                                        1  => break 'case1,
                                                                        _  => break 'outer,
                                                                    }
                                                                } // end 'case15:
                                                                k4 ^= (tail[14] as u32) << 16;
                                                            } // end 'case14:
                                                            k4 ^= (tail[13] as u32) << 8;
                                                        } // end 'case13:
                                                        k4 ^= (tail[12] as u32) << 0;
                                                        k4 = k4.wrapping_mul(c4);
                                                        k4 = rotl32(k4, 18);
                                                        k4 = k4.wrapping_mul(c1);
                                                        h4 ^= k4;
                                                    } // end 'case12:
                                                    k3 ^= (tail[11] as u32) << 24;
                                                } // end 'case11:
                                                k3 ^= (tail[10] as u32) << 16;
                                            } // end 'case10:
                                            k3 ^= (tail[9] as u32) << 8;
                                        } // end 'case9:
                                        k3 ^= (tail[8] as u32) << 0;
                                        k3 = k3.wrapping_mul(c3);
                                        k3 = rotl32(k3, 17);
                                        k3 = k3.wrapping_mul(c4);
                                        h3 ^= k3;
                                    } // end 'case8:
                                    k2 ^= (tail[7] as u32) << 24;
                                } // end 'case7:
                                k2 ^= (tail[6] as u32) << 16;
                            } // end 'case6:
                            k2 ^= (tail[5] as u32) << 8;
                        } // end 'case5:
                        k2 ^= (tail[4] as u32) << 0;
                        k2 = k2.wrapping_mul(c2);
                        k2 = rotl32(k2, 16);
                        k2 = k2.wrapping_mul(c3);
                        h2 ^= k2;
                    } // end 'case4:
                    k1 ^= (tail[3] as u32) << 24;
                } // end 'case3:
                k1 ^= (tail[2] as u32) << 16;
            } // end 'case2:
            k1 ^= (tail[1] as u32) << 8;
        } // end 'case1:
        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = rotl32(k1, 15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    } // end 'outer:
    h1.wrapping_add(h2).wrapping_add(h3).wrapping_add(h4)
}

/// This version implements the same logic using a cascade of if–statements.
#[inline(always)]
fn if_version_inline(
    len: u32,
    mut k1: u32,
    mut k2: u32,
    mut k3: u32,
    mut k4: u32,
    mut h1: u32,
    mut h2: u32,
    mut h3: u32,
    mut h4: u32,
    c1: u32,
    c2: u32,
    c3: u32,
    c4: u32,
    tail: &[u8],
) -> u32 {
    let rem = (len & 15) as usize;
    if rem >= 15 {
        k4 ^= (tail[14] as u32) << 16;
    }
    if rem >= 14 {
        k4 ^= (tail[13] as u32) << 8;
    }
    if rem >= 13 {
        k4 ^= (tail[12] as u32) << 0;
        k4 = k4.wrapping_mul(c4);
        k4 = rotl32(k4, 18);
        k4 = k4.wrapping_mul(c1);
        h4 ^= k4;
    }
    if rem >= 12 {
        k3 ^= (tail[11] as u32) << 24;
    }
    if rem >= 11 {
        k3 ^= (tail[10] as u32) << 16;
    }
    if rem >= 10 {
        k3 ^= (tail[9] as u32) << 8;
    }
    if rem >= 9 {
        k3 ^= (tail[8] as u32) << 0;
        k3 = k3.wrapping_mul(c3);
        k3 = rotl32(k3, 17);
        k3 = k3.wrapping_mul(c4);
        h3 ^= k3;
    }
    if rem >= 8 {
        k2 ^= (tail[7] as u32) << 24;
    }
    if rem >= 7 {
        k2 ^= (tail[6] as u32) << 16;
    }
    if rem >= 6 {
        k2 ^= (tail[5] as u32) << 8;
    }
    if rem >= 5 {
        k2 ^= (tail[4] as u32) << 0;
        k2 = k2.wrapping_mul(c2);
        k2 = rotl32(k2, 16);
        k2 = k2.wrapping_mul(c3);
        h2 ^= k2;
    }
    if rem >= 4 {
        k1 ^= (tail[3] as u32) << 24;
    }
    if rem >= 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if rem >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if rem >= 1 {
        k1 ^= (tail[0] as u32) << 0;
        k1 = k1.wrapping_mul(c1);
        k1 = rotl32(k1, 15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }
    h1.wrapping_add(h2).wrapping_add(h3).wrapping_add(h4)
}

fn bench_break(c: &mut Criterion) {
    // Prepare a tail buffer with 15 bytes (0, 1, 2, … 14)
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("break_version", |b| {
        b.iter(|| {
            // Use arbitrary nonzero values for the variables.
            let _ = break_version(
                len,
                1, 2, 3, 4, // k1, k2, k3, k4
                5, 6, 7, 8, // h1, h2, h3, h4
                9, 10, 11, 12, // c1, c2, c3, c4
                &black_box(tail.clone()),
            );
        })
    });
}

fn bench_if(c: &mut Criterion) {
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("if_version", |b| {
        b.iter(|| {
            let _ = if_version(len, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, &black_box(tail.clone()));
        })
    });
}

fn bench_break_inline(c: &mut Criterion) {
    // Prepare a tail buffer with 15 bytes (0, 1, 2, … 14)
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("break_version_inline", |b| {
        b.iter(|| {
            // Use arbitrary nonzero values for the variables.
            let _ = break_version_inline(
                len,
                1, 2, 3, 4, // k1, k2, k3, k4
                5, 6, 7, 8, // h1, h2, h3, h4
                9, 10, 11, 12, // c1, c2, c3, c4
                &black_box(tail.clone()),
            );
        })
    });
}

fn bench_if_inline(c: &mut Criterion) {
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("if_version_inline", |b| {
        b.iter(|| {
            let _ = if_version_inline(len, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, &black_box(tail.clone()));
        })
    });
}

fn bench_break_noblackbox(c: &mut Criterion) {
    // Prepare a tail buffer with 15 bytes (0, 1, 2, … 14)
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("break_version_noblackbox", |b| {
        b.iter(|| {
            // Use arbitrary nonzero values for the variables.
            let _ = break_version(
                len,
                1, 2, 3, 4, // k1, k2, k3, k4
                5, 6, 7, 8, // h1, h2, h3, h4
                9, 10, 11, 12, // c1, c2, c3, c4
                &tail,
            );
        })
    });
}

fn bench_if_noblackbox(c: &mut Criterion) {
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("if_version_noblackbox", |b| {
        b.iter(|| {
            let _ = if_version(len, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, &tail);
        })
    });
}

fn bench_break_inline_noblackbox(c: &mut Criterion) {
    // Prepare a tail buffer with 15 bytes (0, 1, 2, … 14)
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("break_version_inline_noblackbox", |b| {
        b.iter(|| {
            // Use arbitrary nonzero values for the variables.
            let _ = break_version_inline(
                len,
                1, 2, 3, 4, // k1, k2, k3, k4
                5, 6, 7, 8, // h1, h2, h3, h4
                9, 10, 11, 12, // c1, c2, c3, c4
                &tail,
            );
        })
    });
}

fn bench_if_inline_noblackbox(c: &mut Criterion) {
    let tail: Vec<u8> = (0..15).map(|x| x as u8).collect();
    let len = 15u32;
    c.bench_function("if_version_inline_noblackbox", |b| {
        b.iter(|| {
            let _ = if_version_inline(len, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, &tail);
        })
    });
}

criterion_group!(benches, bench_break, bench_if);
criterion_group!(benches_inline, bench_break_inline, bench_if_inline);

criterion_group!(benches_noblackbox, bench_break_noblackbox, bench_if_noblackbox);
criterion_group!(benches_inline_noblackbox, bench_break_inline_noblackbox, bench_if_inline_noblackbox);

criterion_main!(benches, benches_inline, benches_noblackbox, benches_inline_noblackbox);

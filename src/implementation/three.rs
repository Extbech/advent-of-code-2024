use crate::Solution;
use safe_goto::safe_goto;
pub struct DayThreeSolution {
    data: String,
}

impl Solution for DayThreeSolution {
    const DAY: u8 = 3;

    fn new() -> Self {
        DayThreeSolution {
            data: Self::read_data_to_string().unwrap(),
        }
    }

    fn part_one(&self) -> u32 {
        let mut data = self.data.as_bytes();
        let mut acc = 0;
        safe_goto! {
            begin() {
                if let Some(b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_m() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        b'u' => {goto found_mu()}
                        _ => {goto found_()}
                    }
                }
            },
            found_mu() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        b'l' => {goto found_mul()}
                        _ => {goto found_()}
                    }
                }
            },
            found_mul() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        b'(' => {goto found_mullp(0)}
                        _ => {goto found_()}
                    }
                }
            },
            found_mullp(mut l: u32) {
                data = &data[1..];
                if let Some(&b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        b'0'..=b'9' => {
                            l *= 10;
                            goto found_mullp(l+(b as u32-48))
                        },
                        b',' => {goto found_mulcom(l,0)}
                        _ => {goto found_()}
                    }
                }
            },
            found_mulcom(l: u32, mut r: u32) {
                data = &data[1..];
                if let Some(&b) = data.first() {
                    match b {
                        b'm' => {goto found_m()},
                        b'0'..=b'9' => {
                            r *= 10;
                            goto found_mulcom(l, r+(b as u32-48))
                        },
                        b')' => {
                            acc += l*r;
                            goto found_()
                        }
                        _ => {goto found_()}
                    }
                }
            }
        }
        acc
    }

    fn part_two(&self) -> u32 {
        let mut data = self.data.as_bytes();
        let mut acc = 0;
        safe_goto! {
            begin() {
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_d() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'o' => {goto found_do()},
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_do() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'n' => {goto found_don()}
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_don() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'\'' => {goto found_donap()}
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_donap() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b't' => {goto found_dont()}
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_dont() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'(' => {goto found_dontlp()}
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            found_dontlp() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b')' => {goto dont_found_()}
                        b'm' => {goto found_m()},
                        _ => {goto found_()}
                    }
                }
            },
            dont_found_() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto dont_found_d()},
                        _ => {goto dont_found_()}
                    }
                }
            },
            dont_found_d() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto dont_found_d()},
                        b'o' => {goto dont_found_do()},
                        _ => {goto dont_found_()}
                    }
                }
            },
            dont_found_do() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto dont_found_d()},
                        b'(' => {goto dont_found_dolp()},
                        _ => {goto dont_found_()}
                    }
                }
            },
            dont_found_dolp() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto dont_found_d()},
                        b')' => {goto found_()},
                        _ => {goto dont_found_()}
                    }
                }
            },
            found_m() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        b'u' => {goto found_mu()}
                        _ => {goto found_()}
                    }
                }
            },
            found_mu() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        b'l' => {goto found_mul()}
                        _ => {goto found_()}
                    }
                }
            },
            found_mul() {
                data = &data[1..];
                if let Some(b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        b'(' => {goto found_mullp(0)}
                        _ => {goto found_()}
                    }
                }
            },
            found_mullp(mut l: u32) {
                data = &data[1..];
                if let Some(&b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        b'0'..=b'9' => {
                            l *= 10;
                            goto found_mullp(l+(b as u32-48))
                        },
                        b',' => {goto found_mulcom(l,0)}
                        _ => {goto found_()}
                    }
                }
            },
            found_mulcom(l: u32, mut r: u32) {
                data = &data[1..];
                if let Some(&b) = data.first() {
                    match b {
                        b'd' => {goto found_d()},
                        b'm' => {goto found_m()},
                        b'0'..=b'9' => {
                            r *= 10;
                            goto found_mulcom(l, r+(b as u32-48))
                        },
                        b')' => {
                            acc += l*r;
                            goto found_()
                        }
                        _ => {goto found_()}
                    }
                }
            }
        }
        acc
    }
}

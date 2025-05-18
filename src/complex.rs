use anyhow::{anyhow, Result};
use num::Complex;
use regex::Regex;

// 文字列を Complex<f64> に変換する trait
#[allow(dead_code)]
pub trait Parser {
    fn parse(s: &str) -> Result<Complex<f64>>;
}

impl Parser for &str {
    fn parse(s: &str) -> Result<Complex<f64>> {
        // 複素数(a + bi) の正規表現
        let reg = Regex::new(r#"(?P<real>\d+[.]\d+)[+](?P<im>\d+[.]\d+i)"#).unwrap();

        let matches = reg
            .captures(s)
            .ok_or_else(|| anyhow!("invalid complex format(failed to capture)"))?;
        let real = matches
            .name("real")
            .ok_or_else(|| anyhow!("invalid complex format(<real> not found)"))?
            .as_str();
        let im = matches
            .name("im")
            .ok_or_else(|| anyhow!("invalid complex format(<im> not found)"))?
            .as_str();
        Ok(Complex::new(real.parse()?, im.parse()?))
    }
}

#[cfg(test)]
mod test {
    use num::Complex;

    #[test]
    // parse_complex() に対するユニットテスト
    // 入力(引数)に対する出力(戻り値)が期待するものとなっているかを検証する
    fn test_parse_complex() {
        struct Testcase<'a> {
            input: &'a str,               // 入力値
            want_err: bool,               // Err を期待するか
            expect: Option<Complex<f64>>, // 期待する出力
        }
        let tests = vec![
            Testcase {
                input: "3.14+2.718i",
                want_err: false,
                expect: Some(Complex::new(3.14, 2.718)),
            },
            Testcase {
                input: "2.718i",
                want_err: false,
                expect: Some(Complex::new(0.0, 2.718)),
            },
            Testcase {
                input: "3.14",
                want_err: false,
                expect: Some(Complex::new(3.14, 0.0)),
            },
            Testcase {
                input: "i",
                want_err: true,
                expect: None,
            },
        ];
        for test in tests {
            let res = test.input.parse();
            dbg!(&res);
            assert_eq!(test.want_err, res.is_err());
            if let Ok(res) = res {
                assert_eq!(test.expect.unwrap(), res)
            }
        }
    }
}

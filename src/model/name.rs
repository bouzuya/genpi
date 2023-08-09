use std::collections::HashMap;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Name {
    pub first_name: String,
    pub first_name_kana: String,
    pub last_name: String,
    pub last_name_kana: String,
}

impl Name {
    pub fn in_katakana(self) -> Self {
        Self {
            first_name_kana: Self::hiragana_to_katakana(&self.first_name_kana).unwrap(),
            last_name_kana: Self::hiragana_to_katakana(&self.last_name_kana).unwrap(),
            ..self
        }
    }

    pub fn in_halfwidth_kana(self) -> Self {
        Self {
            first_name_kana: Self::hiragana_to_halfwidth_kana(&self.first_name_kana).unwrap(),
            last_name_kana: Self::hiragana_to_halfwidth_kana(&self.last_name_kana).unwrap(),
            ..self
        }
    }

    fn hiragana_to_halfwidth_kana(s: &str) -> anyhow::Result<String> {
        let map = {
            let mut map = HashMap::new();
            map.insert('ぁ', "ｧ");
            map.insert('あ', "ｱ");
            map.insert('ぃ', "ｨ");
            map.insert('い', "ｲ");
            map.insert('ぅ', "ｩ");
            map.insert('う', "ｳ");
            map.insert('ぇ', "ｪ");
            map.insert('え', "ｴ");
            map.insert('ぉ', "ｫ");
            map.insert('お', "ｵ");
            map.insert('か', "ｶ");
            map.insert('が', "ｶﾞ");
            map.insert('き', "ｷ");
            map.insert('ぎ', "ｷﾞ");
            map.insert('く', "ｸ");
            map.insert('ぐ', "ｸﾞ");
            map.insert('け', "ｹ");
            map.insert('げ', "ｹﾞ");
            map.insert('こ', "ｺ");
            map.insert('ご', "ｺﾞ");
            map.insert('さ', "ｻ");
            map.insert('ざ', "ｻﾞ");
            map.insert('し', "ｼ");
            map.insert('じ', "ｼﾞ");
            map.insert('す', "ｽ");
            map.insert('ず', "ｽﾞ");
            map.insert('せ', "ｾ");
            map.insert('ぜ', "ｾﾞ");
            map.insert('そ', "ｿ");
            map.insert('ぞ', "ｿﾞ");
            map.insert('た', "ﾀ");
            map.insert('だ', "ﾀﾞ");
            map.insert('ち', "ﾁ");
            map.insert('ぢ', "ﾁﾞ");
            map.insert('っ', "ｯ");
            map.insert('つ', "ﾂ");
            map.insert('づ', "ﾂﾞ");
            map.insert('て', "ﾃ");
            map.insert('で', "ﾃﾞ");
            map.insert('と', "ﾄ");
            map.insert('ど', "ﾄﾞ");
            map.insert('な', "ﾅ");
            map.insert('に', "ﾆ");
            map.insert('ぬ', "ﾇ");
            map.insert('ね', "ﾈ");
            map.insert('の', "ﾉ");
            map.insert('は', "ﾊ");
            map.insert('ば', "ﾊﾞ");
            map.insert('ぱ', "ﾊﾟ");
            map.insert('ひ', "ﾋ");
            map.insert('び', "ﾋﾞ");
            map.insert('ぴ', "ﾋﾟ");
            map.insert('ふ', "ﾌ");
            map.insert('ぶ', "ﾌﾞ");
            map.insert('ぷ', "ﾌﾟ");
            map.insert('へ', "ﾍ");
            map.insert('べ', "ﾍﾞ");
            map.insert('ぺ', "ﾍﾟ");
            map.insert('ほ', "ﾎ");
            map.insert('ぼ', "ﾎﾞ");
            map.insert('ぽ', "ﾎﾟ");
            map.insert('ま', "ﾏ");
            map.insert('み', "ﾐ");
            map.insert('む', "ﾑ");
            map.insert('め', "ﾒ");
            map.insert('も', "ﾓ");
            map.insert('ゃ', "ｬ");
            map.insert('や', "ﾔ");
            map.insert('ゅ', "ｭ");
            map.insert('ゆ', "ﾕ");
            map.insert('ょ', "ｮ");
            map.insert('よ', "ﾖ");
            map.insert('ら', "ﾗ");
            map.insert('り', "ﾘ");
            map.insert('る', "ﾙ");
            map.insert('れ', "ﾚ");
            map.insert('ろ', "ﾛ");
            map.insert('ゎ', "ﾜ"); // not found
            map.insert('わ', "ﾜ");
            map.insert('ゐ', "ｲ"); // not found
            map.insert('ゑ', "ｴ"); // not found
            map.insert('を', "ｦ");
            map.insert('ん', "ﾝ");
            map.insert('ゔ', "ｳﾞ");
            map.insert('ゕ', "ｶ"); // not found
            map.insert('ゖ', "ｹ"); // not found
            map
        };
        s.chars().try_fold(String::new(), |mut acc, c: char| {
            let b = c as u32;
            if !(0x3041..=0x3096).contains(&b) {
                Err(anyhow::anyhow!("{} is not hiragana", c))
            } else {
                acc.push_str(map.get(&c).unwrap());
                Ok(acc)
            }
        })
    }

    fn hiragana_to_katakana(s: &str) -> anyhow::Result<String> {
        s.chars()
            .map(|c: char| {
                let b = c as u32;
                if !(0x3041..=0x3096).contains(&b) {
                    Err(anyhow::anyhow!("{} is not hiragana", c))
                } else {
                    Ok(char::from_u32(b + 0x0060).unwrap())
                }
            })
            .collect::<anyhow::Result<String>>()
    }
}

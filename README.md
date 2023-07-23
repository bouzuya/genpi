# genpi

A command-line tool for generating personal information (PI).

## Usage

```console
$ genpi | jq .
{
  "date_of_birth": "2012-07-13",
  "first_name": "敦",
  "first_name_kana": "あつし",
  "last_name": "菊地",
  "last_name_kana": "きくち",
  "sex": "male"
}

$ genpi --katakana | jq .
{
  "date_of_birth": "1939-02-23",
  "first_name": "美緒",
  "first_name_kana": "ミオ",
  "last_name": "植田",
  "last_name_kana": "ウエダ",
  "sex": "female"
}

$ genpi --halfwidth --katakana | jq .
{
  "date_of_birth": "1980-09-18",
  "first_name": "治",
  "first_name_kana": "ｵｻﾑ",
  "last_name": "大貫",
  "last_name_kana": "ｵｵﾇｷ",
  "sex": "male"
}

$ genpi --server
$ # in another terminal
$ curl -s 'http://localhost:3000' | jq .
{
  "date_of_birth": "1912-09-01",
  "first_name": "由子",
  "first_name_kana": "ゆうこ",
  "last_name": "熊谷",
  "last_name_kana": "くまがい",
  "sex": "female"
}
$ curl -s 'http://localhost:3000?katakana=true' | jq .
{
  "date_of_birth": "2014-01-13",
  "first_name": "遥",
  "first_name_kana": "ハルカ",
  "last_name": "中島",
  "last_name_kana": "ナカシマ",
  "sex": "female"
}
$ curl -s 'http://localhost:3000?halfwidth=true&katakana=true' | jq .
{
  "date_of_birth": "1996-04-06",
  "first_name": "美穂",
  "first_name_kana": "ﾐﾎ",
  "last_name": "和田",
  "last_name_kana": "ﾜﾀﾞ",
  "sex": "female"
}
```

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

$ genpi -- --katakana --halfwidth | jq .
{
  "date_of_birth": "1980-09-18",
  "first_name": "治",
  "first_name_kana": "ｵｻﾑ",
  "last_name": "大貫",
  "last_name_kana": "ｵｵﾇｷ",
  "sex": "male"
}
```

# input.txt を読み込む

with open('input.txt', 'r') as file:
    lines = file.readlines()
    # 各行には英単語が入っている
    # 5 文字の単語を抽出する
    five_letter_words = [line.strip() for line in lines if len(line.strip()) == 5]

# 結果を output.json に書き込む
import json
with open('output.json', 'w') as file:
    json.dump(five_letter_words, file, ensure_ascii=False, indent=4)
